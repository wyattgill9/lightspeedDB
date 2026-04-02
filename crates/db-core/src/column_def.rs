use crate::dtype::DataTypeKind;

#[derive(Debug, PartialEq, Eq)]
pub struct ColumnDef {
    data_type: DataTypeKind,
    width: u32,
    name: String,
}

impl ColumnDef {
    pub fn new(name: impl Into<String>, data_type: DataTypeKind) -> Self {
        let name = name.into();
        if name.is_empty() {
            panic!("column name cannot be empty");
        } else {
            Self {
                data_type,
                width: data_type.byte_width() as u32, // @Truncation
                name,
            }
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
