use crate::{column_segment::ColumnSegment, table_schema::TableSchema};

const CAPACITY_ROWS_SEGMENT: u32 = 64 * 2048;

#[derive(Debug)]
pub struct TablePartition {
    columns: Vec<ColumnSegment>,
    row_count: u32,
}

impl TablePartition {
    pub fn new(schema: &TableSchema) -> Self {
        let columns = (0..schema.column_count())
            .map(ColumnSegment::new)
            .collect();

        Self {
            columns,
            row_count: 0,
        }
    }

    /// Insert tightly-packed array-of-structs byte data.
    ///
    /// Each row is `schema.row_byte_width()` bytes wide.
    pub fn insert_rows(&mut self, schema: &TableSchema, bytes: &[u8]) {
        let size_bytes_row = schema.row_byte_width();

        let count_rows_new = if bytes.len().is_multiple_of(size_bytes_row) {
            bytes.len() / size_bytes_row
        } else {
            panic!(
                "row bytes length {} is not a multiple of row size {}",
                bytes.len(),
                size_bytes_row
            );
        };

        let count_rows_available = (CAPACITY_ROWS_SEGMENT - self.row_count) as usize;
        if count_rows_new > count_rows_available {
            panic!("segment full: capacity is {} rows", CAPACITY_ROWS_SEGMENT);
        }

        for row_index in 0..count_rows_new {
            let mut offset = row_index * size_bytes_row;
            for (column_index, column) in schema.columns().iter().enumerate() {
                let width = column.byte_width();
                let end = offset + width;
                self.columns[column_index].append_bytes(&bytes[offset..end]);
                offset = end;
            }
            self.row_count += 1;
        }
    }

    pub fn columns(&self) -> &[ColumnSegment] {
        &self.columns
    }

    pub fn row_count(&self) -> u32 {
        self.row_count
    }
}

