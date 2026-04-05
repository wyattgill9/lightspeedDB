use db_types::DataTypeKind;

pub struct ResultColumn {
    name: String,
    data_type: DataTypeKind,
    data: Vec<u8>,
}

impl ResultColumn {
    pub fn new(name: String, data_type: DataTypeKind) -> Self {
        Self {
            name,
            data_type,
            data: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> DataTypeKind {
        self.data_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn byte_width(&self) -> usize {
        self.data_type.byte_width()
    }

    pub fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }
}

pub struct QueryResult {
    columns: Vec<ResultColumn>,
    row_count: usize,
}

impl QueryResult {
    pub fn new(columns: Vec<ResultColumn>, row_count: usize) -> Self {
        Self { columns, row_count }
    }

    pub fn columns(&self) -> &[ResultColumn] {
        &self.columns
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }
}
