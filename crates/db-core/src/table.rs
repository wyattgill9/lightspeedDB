use crate::dtype::DataTypeKind;

const CAPACITY_ROWS_SEGMENT: u32 = 64 * 2048;

#[derive(Debug)]
pub struct ColumnSegment {
    data: Vec<u8>,
}

impl ColumnSegment {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn append_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Debug)]
pub struct TablePartition {
    columns: Vec<ColumnSegment>,
    column_byte_widths: Vec<usize>,
    row_count: u32,
}

impl TablePartition {
    pub fn new(column_byte_widths: Vec<usize>) -> Self {
        let column_count = column_byte_widths.len();

        let columns = (0..column_count)
                .map(|_| ColumnSegment::new()).collect();

        Self {
            columns,
            column_byte_widths,
            row_count: 0,
        }
    }

    /// Insert tightly-packed array-of-structs byte data.
    ///
    /// Each row is `sum(column_byte_widths)` bytes wide.
    pub fn insert_rows(&mut self, bytes: &[u8]) {
        let size_bytes_row: usize = self.column_byte_widths.iter().sum();

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
            for (column_index, &width) in self.column_byte_widths.iter().enumerate() {
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

#[derive(Debug)]
pub struct DBTable {
    name: String,
    field_names: Vec<String>,
    data_types: Vec<DataTypeKind>,
    row_groups: Vec<TablePartition>,
}

impl DBTable {
    pub fn new(
        name: String,
        field_names: Vec<String>,
        data_types: Vec<DataTypeKind>,
    ) -> Self {
        let column_byte_widths: Vec<usize> = data_types.iter().map(|kind| kind.byte_width()).collect();

        let row_groups = vec![TablePartition::new(column_byte_widths)];

        Self {
            name,
            field_names,
            data_types,
            row_groups,
        }
    }

    pub fn insert(&mut self, bytes: &[u8]) {
        if let Some(row_group) = self.row_groups.last_mut() {
            row_group.insert_rows(bytes);
        } else {
            panic!("table has no row groups: {}", self.name);
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn field_names(&self) -> &[String] {
        &self.field_names
    }

    pub fn data_types(&self) -> &[DataTypeKind] {
        &self.data_types
    }

    pub fn row_groups(&self) -> &[TablePartition] {
        &self.row_groups
    }
}
