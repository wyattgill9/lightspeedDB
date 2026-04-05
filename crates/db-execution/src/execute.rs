use db_catalog::Database;

use crate::physical_plan::PhysicalPlan;
use crate::query_result::{QueryResult, ResultColumn};

pub fn execute(plan: &PhysicalPlan, database: &Database) -> QueryResult {
    match plan {
        PhysicalPlan::TableScan {
            table_name,
            column_indices,
        } => execute_table_scan(database, table_name, column_indices),
    }
}

fn execute_table_scan(
    database: &Database,
    table_name: &str,
    column_indices: &[usize],
) -> QueryResult {
    let table = database.table(table_name);
    let schema = table.schema();

    let mut columns: Vec<ResultColumn> = column_indices
        .iter()
        .map(|&index| {
            let definition = schema.column_at(index);
            ResultColumn::new(definition.name().to_owned(), definition.data_type())
        })
        .collect();

    let mut row_count = 0usize;

    for row_group in table.row_groups() {
        let group_row_count = row_group.row_count() as usize;
        for (result_column, &source_index) in columns.iter_mut().zip(column_indices.iter()) {
            let segment = &row_group.columns()[source_index];
            let byte_width = result_column.byte_width();
            let byte_count = group_row_count * byte_width;
            result_column.extend_from_slice(&segment.data()[..byte_count]);
        }
        row_count += group_row_count;
    }

    QueryResult::new(columns, row_count)
}
