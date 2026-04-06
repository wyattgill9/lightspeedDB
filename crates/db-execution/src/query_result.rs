use db_types::DataTypeKind;

pub struct ResultColumnChunk<'a> {
    data: &'a [u8],
    row_count: usize,
}

impl<'a> ResultColumnChunk<'a> {
    fn new(data: &'a [u8], row_count: usize) -> Self {
        Self { data, row_count }
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }
}

pub struct ResultColumn<'a> {
    name: String,
    data_type: DataTypeKind,
    chunks: Vec<ResultColumnChunk<'a>>,
}

impl<'a> ResultColumn<'a> {
    pub fn new(name: String, data_type: DataTypeKind) -> Self {
        Self {
            name,
            data_type,
            chunks: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> DataTypeKind {
        self.data_type
    }

    pub fn chunks(&self) -> &[ResultColumnChunk<'a>] {
        &self.chunks
    }

    pub fn byte_width(&self) -> usize {
        self.data_type.byte_width()
    }

    pub fn row_bytes(&self, row_index: usize) -> &'a [u8] {
        let byte_width = self.byte_width();
        let mut row_offset = row_index;

        for chunk in &self.chunks {
            if row_offset < chunk.row_count {
                let start = row_offset * byte_width;
                let end = start + byte_width;
                return &chunk.data[start..end];
            }
            row_offset -= chunk.row_count;
        }

        panic!("row index {row_index} out of bounds for column {}", self.name);
    }

    pub fn push_chunk(&mut self, bytes: &'a [u8], row_count: usize) {
        if row_count == 0 {
            return;
        }

        debug_assert_eq!(bytes.len(), row_count * self.byte_width());
        self.chunks.push(ResultColumnChunk::new(bytes, row_count));
    }
}

pub struct QueryResult<'a> {
    columns: Vec<ResultColumn<'a>>,
    row_count: usize,
}

impl<'a> QueryResult<'a> {
    pub fn new(columns: Vec<ResultColumn<'a>>, row_count: usize) -> Self {
        Self { columns, row_count }
    }

    pub fn columns(&self) -> &[ResultColumn<'a>] {
        &self.columns
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }
}
