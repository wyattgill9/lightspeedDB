use lsdb_types::TableSchema;

use crate::ColumnSegmentStatistics;

#[derive(Debug)]
pub struct ColumnSegment {
    data: Vec<u8>,
    column_def_index: usize,
    stats: ColumnSegmentStatistics,
}

impl ColumnSegment {
    pub fn new(column_index: usize) -> Self {
        Self {
            data: Vec::new(),
            column_def_index: column_index,
            stats: ColumnSegmentStatistics::new(),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    pub fn push_dtype_val(&mut self, bytes: &[u8], schema: &TableSchema) {
        self.stats.update(bytes, schema, self.column_def_index);
        self.data.extend_from_slice(bytes);
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn materialize_f32(&self) -> f32 {
        0.0
    }
}
