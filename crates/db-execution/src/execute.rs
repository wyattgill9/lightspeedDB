use db_catalog::Database;

use db_types::PhysicalPlan;
use crate::query_result::{QueryResult, ResultColumn};

pub fn execute<'db>(plan: &PhysicalPlan, database: &'db Database) -> QueryResult<'db> {
    match plan {
        PhysicalPlan::TableScan {
            table_name,
            column_indices,
        } => execute_table_scan(database, table_name, column_indices),
    }
}

fn execute_table_scan<'db>(
    database: &'db Database,
    table_name: &str,
    column_indices: &[usize],
) -> QueryResult<'db> {
    let table = database.table(table_name);
    let schema = table.schema();

    let mut columns: Vec<ResultColumn<'db>> = column_indices
        .iter()
        .map(|&index| {
            let definition = schema.column_at(index);
            ResultColumn::new(definition.name().to_owned(), definition.data_type())
        })
        .collect();

    let mut row_count = 0usize;

    for table_partition in table.table_partitions() {
        let partition_row_count = table_partition.row_count();
        if partition_row_count == 0 {
            continue;
        }

        for (result_column, &source_index) in columns.iter_mut().zip(column_indices.iter()) {
            let segment = &table_partition.columns()[source_index];
            let byte_width = result_column.byte_width();
            let byte_count = partition_row_count * byte_width;
            result_column.push_chunk(&segment.data()[..byte_count], partition_row_count);
        }
        row_count += partition_row_count;
    }

    QueryResult::new(columns, row_count)
}
