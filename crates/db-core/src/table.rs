use crate::dtype::DataTypeKind;
use crate::error::{self, Result};

/// Maximum rows per segment. Safety bound to prevent unbounded growth.
const CAPACITY_ROWS_SEGMENT: u32 = 65_536;

/// Unique identifier for a table within the database.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableId(u32);

impl TableId {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(self) -> u32 {
        self.0
    }

    /// Return the next sequential identifier.
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// A single column's raw byte storage within a segment.
#[derive(Debug, Clone)]
pub struct ColumnSegment {
    data: Vec<u8>,
}

impl ColumnSegment {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Append raw bytes for one or more values in this column.
    fn append_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// A group of column segments forming a horizontal partition of a table.
#[derive(Debug, Clone)]
pub struct TableSegment {
    columns: Vec<ColumnSegment>,
    column_byte_widths: Vec<usize>,
    row_count: u32,
}

impl TableSegment {
    /// Create a new empty segment with the given column byte widths.
    pub fn new(column_byte_widths: Vec<usize>) -> Self {
        let column_count = column_byte_widths.len();
        let columns: Vec<ColumnSegment> = (0..column_count).map(|_| ColumnSegment::new()).collect();
        Self {
            columns,
            column_byte_widths,
            row_count: 0,
        }
    }

    /// Insert tightly-packed array-of-structs byte data.
    ///
    /// Each row is `sum(column_byte_widths)` bytes wide.
    pub fn insert_rows(&mut self, bytes: &[u8]) -> Result<()> {
        let size_bytes_row: usize = self.column_byte_widths.iter().sum();

        let count_rows_new = if bytes.len().is_multiple_of(size_bytes_row) {
            bytes.len() / size_bytes_row
        } else {
            return error::InvalidRowBytesSnafu {
                length_bytes_actual: bytes.len(),
                size_bytes_row,
            }
            .fail();
        };

        for row_index in 0..count_rows_new {
            if self.row_count >= CAPACITY_ROWS_SEGMENT {
                return error::SegmentCapacityExceededSnafu {
                    capacity_rows: CAPACITY_ROWS_SEGMENT,
                }
                .fail();
            }
            let mut offset = row_index * size_bytes_row;
            for (column_index, &width) in self.column_byte_widths.iter().enumerate() {
                let end = offset + width;
                self.columns[column_index].append_bytes(&bytes[offset..end]);
                offset = end;
            }
            self.row_count += 1;
        }

        Ok(())
    }

    pub fn columns(&self) -> &[ColumnSegment] {
        &self.columns
    }

    pub fn row_count(&self) -> u32 {
        self.row_count
    }
}

/// Core table structure holding schema and row group data.
#[derive(Debug, Clone)]
pub struct DatabaseTable {
    id: TableId,
    name: String,
    field_names: Vec<String>,
    data_types: Vec<DataTypeKind>,
    row_groups: Vec<TableSegment>,
}

impl DatabaseTable {
    /// Create a new table with one empty row group.
    pub fn new(
        id: TableId,
        name: String,
        field_names: Vec<String>,
        data_types: Vec<DataTypeKind>,
    ) -> Self {
        let column_byte_widths: Vec<usize> =
            data_types.iter().map(|kind| kind.byte_width()).collect();
        let row_groups = vec![TableSegment::new(column_byte_widths)];
        Self {
            id,
            name,
            field_names,
            data_types,
            row_groups,
        }
    }

    /// Insert tightly-packed row data into the active (last) row group.
    pub fn insert(&mut self, bytes: &[u8]) -> Result<()> {
        // row_groups is always non-empty: initialized with one segment in new().
        self.row_groups.last_mut().unwrap().insert_rows(bytes)
    }

    pub fn id(&self) -> TableId {
        self.id
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

    pub fn row_groups(&self) -> &[TableSegment] {
        &self.row_groups
    }
}
