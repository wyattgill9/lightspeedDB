use crate::table_meta::TableMeta;
use crate::table_partition::TablePartition;
use crate::table_schema::TableSchema;
use crate::table_stats::TableStatistics;

const CAPACITY_ROWS_WRITE_BUFFER: usize = 4096; /// Rows accumulate here before being flushed to a partition in one batch.

#[derive(Debug)]
pub struct DBTable {
    meta: TableMeta,
    schema: TableSchema,
    row_groups: Vec<TablePartition>,
    stats: TableStatistics,

    write_buffer: Vec<u8>,
}

impl DBTable {
    pub fn new(name: String, id: u32, schema: TableSchema) -> Self {
        let row_groups = vec![TablePartition::new(&schema)];
        let buffer_capacity_bytes = CAPACITY_ROWS_WRITE_BUFFER * schema.row_size_bytes();

        Self {
            meta: TableMeta::new(name, id),
            schema,
            row_groups,
            stats: TableStatistics::new(),
            write_buffer: Vec::with_capacity(buffer_capacity_bytes),
        }
    }

    /// Buffer rows; flush to partitions automatically when the buffer reaches capacity.
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
                self.flush();
            }
        }
    }

    /// Drain any buffered rows into row group partitions.
    pub fn flush(&mut self) {
        if self.write_buffer.is_empty() {
            return;
        }
        // Swap out the buffer so we can call write_rows_to_partitions (which needs &mut self)
        // while holding the byte slice. Swap back afterward to reuse the allocation.
        let mut buffer = Vec::new();
        std::mem::swap(&mut self.write_buffer, &mut buffer);
        self.write_rows_to_partitions(&buffer);
        buffer.clear();
        std::mem::swap(&mut self.write_buffer, &mut buffer);
    }

    fn write_rows_to_partitions(&mut self, bytes: &[u8]) {
        let row_byte_width = self.schema.row_size_bytes();
        let row_count_total = bytes.len() / row_byte_width;
        let mut row_count_done = 0usize;

        while row_count_done < row_count_total {
            let is_full = self
                .row_groups
                .last()
                .is_none_or(|rg| rg.rows_available() == 0);

            if is_full {
                self.row_groups.push(TablePartition::new(&self.schema));
            } else {
                // current row group has capacity; no action needed
            }

            let schema = &self.schema;
            let row_group = self.row_groups.last_mut().expect("row group exists");

            let row_count_chunk =
                (row_count_total - row_count_done).min(row_group.rows_available() as usize);
            let byte_start = row_count_done * row_byte_width;
            let byte_end = byte_start + row_count_chunk * row_byte_width;
            row_group.insert_rows(schema, &bytes[byte_start..byte_end]);
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

    pub fn row_groups(&self) -> &[TablePartition] {
        &self.row_groups
    }

    pub fn stats(&self) -> &TableStatistics {
        &self.stats
    }
}
