use crate::column_def::ColumnDef;

#[derive(Debug, PartialEq, Eq)]
pub struct TableSchema {
    columns: Vec<ColumnDef>,
    row_width_bytes: usize,
}

impl TableSchema {
    pub fn new(columns: Vec<ColumnDef>) -> Self {
        if columns.is_empty() {
            panic!("table schema must contain at least one column");
        } else {
            let mut row_byte_width = 0usize;

            for column in &columns {
                row_byte_width = row_byte_width
                    .checked_add(column.byte_width())
                    .unwrap_or_else(|| panic!("schema row width overflowed"));
            }

            Self {
                columns,
                row_width_bytes: row_byte_width,
            }
        }
    }

    pub fn from_fields(fields: &[(&str, &str)]) -> Self {
        let columns = fields
            .iter()
            .map(|(name, type_name)| ColumnDef::from_type_name(*name, type_name))
            .collect();

        Self::new(columns)
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn columns(&self) -> &[ColumnDef] {
        &self.columns
    }

    pub fn row_byte_width(&self) -> usize {
        self.row_width_bytes
    }
}


impl std::ops::Index<usize> for TableSchema {
    type Output = ColumnDef;

    fn index(&self, index: usize) -> &Self::Output {
        &self.columns[index]
    }
}
