use crate::{column_segment::ColumnSegment, table_schema::TableSchema};

const CAPACITY_ROWS_SEGMENT: u32 = 64 * 2048;

#[derive(Debug)]
pub struct TablePartition {
    columns: Vec<ColumnSegment>,
    row_count: u32,
}

impl TablePartition {
    pub fn new(schema: &TableSchema) -> Self {
        let columns = (0..schema.column_count()).map(ColumnSegment::new).collect();

        Self {
            columns,
            row_count: 0,
        }
    }

    /// Insert tightly-packed array-of-structs byte data.
    ///
    /// Caller must pre-validate: `bytes.len()` is a multiple of `schema.row_byte_width()`,
    /// and the row count does not exceed `rows_available()`.
    pub fn insert_rows(&mut self, schema: &TableSchema, bytes: &[u8]) {
        let row_byte_width = schema.row_size_bytes();
        let row_count = bytes.len() / row_byte_width;

        if row_count > self.rows_available() as usize {
            panic!(
                "row group overflow: {} rows requested, {} available",
                row_count,
                self.rows_available()
            );
        } else {
            for row_bytes in bytes.chunks_exact(row_byte_width) {
                let mut byte_offset = 0usize;
                for (column_index, column) in schema.columns().iter().enumerate() {
                    let byte_end = byte_offset + column.byte_width();
                    self.columns[column_index].append_bytes(&row_bytes[byte_offset..byte_end]);
                    byte_offset = byte_end;
                }
                self.row_count += 1;
            }
        }
    }

    pub fn columns(&self) -> &[ColumnSegment] {
        &self.columns
    }

    pub fn row_count(&self) -> u32 {
        self.row_count
    }

    pub fn rows_available(&self) -> u32 {
        CAPACITY_ROWS_SEGMENT - self.row_count
    }
}
