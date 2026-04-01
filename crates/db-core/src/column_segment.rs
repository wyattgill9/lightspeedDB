use crate::column_def::ColumnDef;
use crate::table_schema::TableSchema;

#[derive(Debug)]
pub struct ColumnSegment {
    data: Vec<u8>,
    column_def_index: usize,
}

impl ColumnSegment {
    pub fn new(column_index: usize) -> Self {
        Self {
            data: Vec::new(),
            column_def_index: column_index,
        }
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn def<'s>(&self, schema: &'s TableSchema) -> &'s ColumnDef {
        &schema[self.column_def_index]
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
