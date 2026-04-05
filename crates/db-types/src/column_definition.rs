use crate::data_type::DataTypeKind;

#[derive(Debug, PartialEq, Eq)]
pub struct ColumnDefinition {
    dtype: DataTypeKind,
    width: u32,
    name: String,
}

impl ColumnDefinition {
    pub fn new(name: impl Into<String>, data_type: DataTypeKind) -> Self {
        let name = name.into();
        if name.is_empty() {
            panic!("column name cannot be empty");
        } else {
            Self {
                dtype: data_type,
                width: data_type.byte_width() as u32, // @Truncation
                name,
            }
        }
    }

    pub fn from_type_name(name: impl Into<String>, type_name: &str) -> Self {
        Self::new(name, DataTypeKind::parse(type_name))
    }

    pub fn byte_width(&self) -> usize {
        self.dtype.byte_width()
    }

    pub fn data_type(&self) -> DataTypeKind {
        self.dtype
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
