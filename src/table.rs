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
        debug_assert!(self.0 < u32::MAX, "Table identifier must not overflow.");
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
        let segment = Self { data: Vec::new() };
        debug_assert!(segment.data.is_empty(), "New column must start empty.");
        debug_assert_eq!(
            segment.data.capacity(),
            0,
            "New column must not preallocate."
        );
        segment
    }

    /// Append raw bytes for one or more values in this column.
    fn append_bytes(&mut self, bytes: &[u8]) {
        assert!(!bytes.is_empty(), "Cannot append empty byte slice.");
        let length_before = self.data.len();
        self.data.extend_from_slice(bytes);
        debug_assert_eq!(
            self.data.len(),
            length_before + bytes.len(),
            "Data length must grow by appended byte count."
        );
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
        assert!(
            !column_byte_widths.is_empty(),
            "Segment must have at least one column."
        );
        assert!(
            column_byte_widths.iter().all(|&width| width > 0),
            "All column byte widths must be positive."
        );

        let column_count = column_byte_widths.len();
        let columns: Vec<ColumnSegment> = (0..column_count).map(|_| ColumnSegment::new()).collect();
        debug_assert_eq!(
            columns.len(),
            column_count,
            "Column count must match width count."
        );

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
        assert!(!bytes.is_empty(), "Insert data must not be empty.");
        let size_bytes_row: usize = self.column_byte_widths.iter().sum();
        more_asserts::assert_gt!(size_bytes_row, 0, "Row byte size must be positive.");

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
            assert!(
                self.row_count < CAPACITY_ROWS_SEGMENT,
                "Segment row count must not exceed capacity."
            );
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
        assert_eq!(
            field_names.len(),
            data_types.len(),
            "Field name count must equal data type count."
        );
        assert!(
            !field_names.is_empty(),
            "Table must have at least one field."
        );

        let column_byte_widths: Vec<usize> =
            data_types.iter().map(|kind| kind.byte_width()).collect();
        let initial_segment = TableSegment::new(column_byte_widths);
        let row_groups = vec![initial_segment];

        debug_assert_eq!(
            row_groups.len(),
            1,
            "New table must have exactly one row group."
        );

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
        assert!(!bytes.is_empty(), "Insert data must not be empty.");
        assert!(
            !self.row_groups.is_empty(),
            "Table must have at least one row group."
        );
        // The assertion above guarantees last_mut() succeeds.
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
