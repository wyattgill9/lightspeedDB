use crate::query_result::QueryResult;

/// Formatted text representation of a query result.
pub struct OutputTable {
    output: String,
}

impl std::fmt::Display for OutputTable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.output)
    }
}

impl OutputTable {
    pub fn from_query_result(result: &QueryResult) -> Self {
        let row_count = result.row_count();
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(row_count);

        for row_index in 0..row_count {
            let cells: Vec<String> = result
                .columns()
                .iter()
                .map(|column| {
                    let width = column.byte_width();
                    let start = row_index * width;
                    let end = start + width;
                    column.data_type().format_bytes(&column.data()[start..end])
                })
                .collect();
            rows.push(cells);
        }

        let headers: Vec<String> = result
            .columns()
            .iter()
            .map(|column| column.name().to_owned())
            .collect();

        let mut column_widths: Vec<usize> = headers.iter().map(|header| header.len()).collect();
        for row in &rows {
            for (index, cell) in row.iter().enumerate() {
                column_widths[index] = column_widths[index].max(cell.len());
            }
        }

        let separator = format_separator(&column_widths);
        let mut output = String::new();
        output.push('\n');
        output.push_str(&separator);
        output.push('\n');
        output.push_str(&format_row(&headers, &column_widths));
        output.push('\n');
        output.push_str(&separator);
        output.push('\n');
        for row in &rows {
            output.push_str(&format_row(row, &column_widths));
            output.push('\n');
        }
        output.push_str(&separator);
        output.push('\n');

        OutputTable { output }
    }
}

/// Build an ASCII separator line from column widths.
fn format_separator(column_widths: &[usize]) -> String {
    let inner: String = column_widths
        .iter()
        .map(|&width| "-".repeat(width + 2))
        .collect::<Vec<_>>()
        .join("+");

    format!("+{inner}+")
}

/// Format a single row of cells padded to column widths.
fn format_row(cells: &[String], column_widths: &[usize]) -> String {
    let inner: String = cells
        .iter()
        .enumerate()
        .map(|(index, cell)| format!(" {:width$} ", cell, width = column_widths[index]))
        .collect::<Vec<_>>()
        .join("|");

    format!("|{inner}|")
}
