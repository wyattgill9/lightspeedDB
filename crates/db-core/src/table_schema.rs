use crate::dtype::DataTypeKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnDef {
    data_type: DataTypeKind,
    name: String,
}

impl ColumnDef {
    pub fn new(name: impl Into<String>, data_type: DataTypeKind) -> Self {
        let name = name.into();
        if name.is_empty() {
            panic!("column name cannot be empty");
        } else {
            Self { data_type, name }
        }
    }

    pub fn from_type_name(name: impl Into<String>, type_name: &str) -> Self {
        Self::new(name, DataTypeKind::parse(type_name))
    }

    pub fn byte_width(&self) -> usize {
        self.data_type.byte_width()
    }

    pub fn data_type(&self) -> DataTypeKind {
        self.data_type
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldDef<'a> {
    pub name: &'a str,
    pub type_name: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableSchema {
    columns: Vec<ColumnDef>,
    row_byte_width: usize,
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
                row_byte_width,
            }
        }
    }

    pub fn from_field_defs(fields: &[FieldDef<'_>]) -> Self {
        let columns = fields
            .iter()
            .map(|field| ColumnDef::from_type_name(field.name, field.type_name))
            .collect();

        Self::new(columns)
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
        self.row_byte_width
    }
}

#[cfg(test)]
mod tests {
    use super::{ColumnDef, TableSchema};
    use crate::dtype::DataTypeKind;

    #[test]
    fn schema_tracks_layout_metadata() {
        let schema = TableSchema::new(vec![
            ColumnDef::new("id", DataTypeKind::U64),
            ColumnDef::new("active", DataTypeKind::BOOL),
        ]);

        assert_eq!(schema.column_count(), 2);
        assert_eq!(schema.row_byte_width(), 9);
        assert_eq!(schema.columns()[0].name(), "id");
        assert_eq!(schema.columns()[1].data_type(), DataTypeKind::BOOL);
    }

    #[test]
    #[should_panic(expected = "duplicate column name in schema")]
    fn schema_rejects_duplicate_column_names() {
        let _schema = TableSchema::new(vec![
            ColumnDef::new("id", DataTypeKind::U64),
            ColumnDef::new("id", DataTypeKind::U32),
        ]);
    }
}
