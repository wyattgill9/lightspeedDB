use crate::column_definition::ColumnDefinition;

#[derive(Debug, PartialEq, Eq)]
pub struct TableSchema {
    columns: Vec<ColumnDefinition>,
    row_size_bytes: usize,
}

impl TableSchema {
    pub fn new(columns: Vec<ColumnDefinition>) -> Self {
        if columns.is_empty() {
            panic!("table schema must contain at least one column");
        } else {
            let mut row_size_bytes = 0usize;

            for column in &columns {
                row_size_bytes = row_size_bytes
                    .checked_add(column.byte_width())
                    .unwrap_or_else(|| panic!("schema row width overflowed"));
            }

            Self {
                columns,
                row_size_bytes,
            }
        }
    }

    pub fn from_fields(fields: &[(&str, &str)]) -> Self {
        let columns = fields
            .iter()
            .map(|(name, type_name)| ColumnDefinition::from_type_name(*name, type_name))
            .collect();

        Self::new(columns)
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn columns(&self) -> &[ColumnDefinition] {
        &self.columns
    }

    pub fn row_size_bytes(&self) -> usize {
        self.row_size_bytes
    }

    pub fn column_at(&self, index: usize) -> &ColumnDefinition {
        &self.columns[index]
    }
}
