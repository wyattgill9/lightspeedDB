use db_storage::TableParitition;
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
    table_parititions: Vec<TableParitition>,
    stats: TableStatistics,

    write_buffer: Vec<u8>,
}

impl DBTable {
    pub fn new(name: String, id: u32, schema: TableSchema) -> Self {
        let table_parititions = vec![TableParitition::new(&schema)];
        let buffer_capacity_bytes = CAPACITY_ROWS_WRITE_BUFFER * schema.row_size_bytes();

        Self {
            meta: TableMeta::new(name, id),
            schema,
            table_parititions,
            stats: TableStatistics::new(),
            write_buffer: Vec::with_capacity(buffer_capacity_bytes),
        }
    }

    /// Buffer rows; flush to table parititions automatically when the
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

    /// Drain any buffered rows into table parititions.
    pub fn flush_write_buffer(&mut self) {
        if self.write_buffer.is_empty() {
            return;
        }
        let mut buffer = Vec::new();
        std::mem::swap(&mut self.write_buffer, &mut buffer);
        self.write_rows_to_table_parititions(&buffer);
        buffer.clear();
        std::mem::swap(&mut self.write_buffer, &mut buffer);
    }

    fn write_rows_to_table_parititions(&mut self, bytes: &[u8]) {
        let row_byte_width = self.schema.row_size_bytes();
        let row_count_total = bytes.len() / row_byte_width;
        let mut row_count_done = 0usize;

        while row_count_done < row_count_total {
            let is_full = self
                .table_parititions
                .last()
                .is_none_or(|table_paritition| table_paritition.rows_available() == 0);

            if is_full {
                self.table_parititions
                    .push(TableParitition::new(&self.schema));
            } else {
                // Current table paritition has capacity; no action needed.
            }

            let schema = &self.schema;
            let table_paritition = self
                .table_parititions
                .last_mut()
                .expect("table paritition exists");

            let row_count_chunk = (row_count_total - row_count_done)
                .min(table_paritition.rows_available() as usize);
            let byte_start = row_count_done * row_byte_width;
            let byte_end = byte_start + row_count_chunk * row_byte_width;
            table_paritition.insert_rows(schema, &bytes[byte_start..byte_end]);
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

    pub fn table_parititions(&self) -> &[TableParitition] {
        &self.table_parititions
    }

    pub fn stats(&self) -> &TableStatistics {
        &self.stats
    }
}
