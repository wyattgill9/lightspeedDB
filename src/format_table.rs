use crate::table::DatabaseTable;

/// Formatted text representation of a table query result.
pub struct OutputTable {
    output: String,
}

impl std::fmt::Display for OutputTable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.output)
    }
}

impl OutputTable {
    /// Build a formatted ASCII table from all row groups.
    pub fn from_table(table: &DatabaseTable) -> Self {
        assert!(
            !table.field_names().is_empty(),
            "Table must have fields to format."
        );
        assert_eq!(
            table.field_names().len(),
            table.data_types().len(),
            "Field count must equal data type count."
        );

        // Decode every segment into flat rows (each row = Vec<String>).
        let mut rows: Vec<Vec<String>> = Vec::new();
        for segment in table.row_groups() {
            let count = segment.row_count() as usize;
            for row_index in 0..count {
                let cells: Vec<String> = table
                    .data_types()
                    .iter()
                    .enumerate()
                    .map(|(column_index, data_type)| {
                        let width = data_type.byte_width();
                        let start = row_index * width;
                        let end = start + width;
                        let column_data = segment.columns()[column_index].data();
                        data_type.format_bytes(&column_data[start..end])
                    })
                    .collect();
                rows.push(cells);
            }
        }

        // Column widths: at least as wide as the header.
        let headers = table.field_names();
        let mut column_widths: Vec<usize> = headers.iter().map(|header| header.len()).collect();
        for row in &rows {
            for (index, cell) in row.iter().enumerate() {
                column_widths[index] = column_widths[index].max(cell.len());
            }
        }

        // Render the formatted output.
        let separator = format_separator(&column_widths);
        let mut output = String::new();
        output.push('\n');
        output.push_str(&format!("Table: {}\n", table.name()));
        output.push_str(&separator);
        output.push('\n');
        output.push_str(&format_row(headers, &column_widths));
        output.push('\n');
        output.push_str(&separator);
        output.push('\n');
        for row in &rows {
            output.push_str(&format_row(row, &column_widths));
            output.push('\n');
        }
        output.push_str(&separator);
        output.push('\n');

        debug_assert!(!output.is_empty(), "Formatted output must not be empty.");
        OutputTable { output }
    }
}

/// Build an ASCII separator line from column widths.
fn format_separator(column_widths: &[usize]) -> String {
    assert!(
        !column_widths.is_empty(),
        "Must have at least one column width."
    );
    let inner: String = column_widths
        .iter()
        .map(|&width| "-".repeat(width + 2))
        .collect::<Vec<_>>()
        .join("+");
    let result = format!("+{inner}+");
    debug_assert!(!result.is_empty(), "Separator must not be empty.");
    result
}

/// Format a single row of cells padded to column widths.
fn format_row(cells: &[String], column_widths: &[usize]) -> String {
    assert_eq!(
        cells.len(),
        column_widths.len(),
        "Cell count must equal column width count."
    );
    debug_assert!(!cells.is_empty(), "Row must have at least one cell.");
    let inner: String = cells
        .iter()
        .enumerate()
        .map(|(index, cell)| format!(" {:width$} ", cell, width = column_widths[index]))
        .collect::<Vec<_>>()
        .join("|");
    format!("|{inner}|")
}
