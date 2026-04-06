use db_storage::TablePartition;
use db_types::TableSchema;

use crate::statistics::TableStatistics;

const CAPACITY_ROWS_WRITE_BUFFER: usize = 4096;

#[derive(Debug)]
pub struct TableMeta {
    name: String,
    id: u32,
}

impl TableMeta {
    pub fn new(name: String, id: u32) -> Self {
        Self { name, id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
pub struct DBTable {
    meta: TableMeta,
    schema: TableSchema,
    table_partitions: Vec<TablePartition>,
    stats: TableStatistics,

    write_buffer: Vec<u8>,
}

impl DBTable {
    pub fn new(name: String, id: u32, schema: TableSchema) -> Self {
        let table_partitions = vec![TablePartition::new(&schema)];
        let buffer_capacity_bytes = CAPACITY_ROWS_WRITE_BUFFER * schema.row_size_bytes();

        Self {
            meta: TableMeta::new(name, id),
            schema,
            table_partitions,
            stats: TableStatistics::new(),
            write_buffer: Vec::with_capacity(buffer_capacity_bytes),
        }
    }

    /// Buffer rows; flush to table partitions automatically when the
    /// buffer reaches capacity.
    pub fn insert(&mut self, bytes: &[u8]) {
        let row_byte_width = self.schema.row_size_bytes();

        if !bytes.len().is_multiple_of(row_byte_width) {
            panic!(
                "byte length {} is not a multiple of row width {}",
                bytes.len(),
                row_byte_width
            );
        } else {
            self.write_buffer.extend_from_slice(bytes);
            let capacity_bytes = CAPACITY_ROWS_WRITE_BUFFER * row_byte_width;
            if self.write_buffer.len() >= capacity_bytes {
                self.flush_write_buffer();
            }
        }
    }

    /// Drain any buffered rows into table partitions.
    pub fn flush_write_buffer(&mut self) {
        if self.write_buffer.is_empty() {
            return;
        }
        let mut buffer = Vec::new();
        std::mem::swap(&mut self.write_buffer, &mut buffer);
        self.write_rows_to_table_partitions(&buffer);
        buffer.clear();
        std::mem::swap(&mut self.write_buffer, &mut buffer);
    }

    fn write_rows_to_table_partitions(&mut self, bytes: &[u8]) {
        let row_byte_width = self.schema.row_size_bytes();
        let row_count_total = bytes.len() / row_byte_width;
        let mut row_count_done = 0usize;

        while row_count_done < row_count_total {
            let is_full = self
                .table_partitions
                .last()
                .is_none_or(|table_partition| table_partition.rows_available() == 0);

            if is_full {
                self.table_partitions
                    .push(TablePartition::new(&self.schema));
            } else {
                // Current table partition has capacity; no action needed.
            }

            let schema = &self.schema;
            let table_partition = self
                .table_partitions
                .last_mut()
                .expect("table partition exists");

            let row_count_chunk = (row_count_total - row_count_done)
                .min(table_partition.rows_available());
            let byte_start = row_count_done * row_byte_width;
            let byte_end = byte_start + row_count_chunk * row_byte_width;
            table_partition.insert_rows(schema, &bytes[byte_start..byte_end]);
            row_count_done += row_count_chunk;
        }
    }

    pub fn name(&self) -> &str {
        self.meta.name()
    }

    pub fn id(&self) -> u32 {
        self.meta.id()
    }

    pub fn schema(&self) -> &TableSchema {
        &self.schema
    }

    pub fn table_partitions(&self) -> &[TablePartition] {
        &self.table_partitions
    }

    pub fn stats(&self) -> &TableStatistics {
        &self.stats
    }
}
