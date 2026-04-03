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
            // Column-major transposition: iterate columns outer, rows inner.
            // Each column buffer stays hot in cache for its entire fill,
            // rather than alternating between all N buffers per row.
            let mut col_byte_start = 0usize;
            for (col_index, col_def) in schema.columns().iter().enumerate() {
                let col_byte_width = col_def.byte_width();
                let col_byte_end = col_byte_start + col_byte_width;
                let col = &mut self.columns[col_index];
                col.reserve(row_count * col_byte_width);
                for row_bytes in bytes.chunks_exact(row_byte_width) {
                    col.push_dtype_val(&row_bytes[col_byte_start..col_byte_end], schema);
                }
                col_byte_start = col_byte_end;
            }
            self.row_count += u32::try_from(row_count).expect("row count fits u32");
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
