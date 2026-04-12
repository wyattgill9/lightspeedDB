use db_types::plan::{
    AggregateExpr, AggregateFunction, ColumnRef, ProjectionExpr, ResolvedAggregateFunction,
    ResolvedColumn, ResolvedExpr, ResolvedSelectItem, ResolvedTable, ScanColumn,
};
use db_types::{LogicalPlan, ResolvedPlan};

pub fn build_plan(resolved: ResolvedPlan) -> LogicalPlan {
    let ResolvedPlan::Select {
        projection,
        from,
        group_by,
    } = resolved;

    build_select(projection, from, group_by)
}

fn build_aggregate_projection(
    projection: Vec<ResolvedSelectItem>,
    group_by: &[ResolvedExpr],
    group_key_count: usize,
) -> Vec<ProjectionExpr> {
    let mut expressions = Vec::with_capacity(projection.len());
    let mut aggregate_ordinal = 0usize;

    for item in projection {
        let output_index = match &item.expr {
            ResolvedExpr::Column(column) => find_group_key_position(column, group_by),
            ResolvedExpr::Aggregate(_) => {
                let index = group_key_count + aggregate_ordinal;
                aggregate_ordinal += 1;
                index
            }
        };

        expressions.push(ProjectionExpr {
            input: ColumnRef {
                index: output_index,
            },
            alias: item.alias,
        });
    }

    expressions
}

fn build_aggregate_query(
    projection: Vec<ResolvedSelectItem>,
    group_by: Vec<ResolvedExpr>,
    table_scan: LogicalPlan,
    referenced: &[usize],
) -> LogicalPlan {
    let group_keys = build_group_keys(&group_by, referenced);
    let aggregates = build_aggregates(&projection, referenced);
    let group_key_count = group_keys.len();

    let aggregate = LogicalPlan::Aggregate {
        group_keys,
        aggregates,
        input: Box::new(table_scan),
    };

    let expressions = build_aggregate_projection(projection, &group_by, group_key_count);

    LogicalPlan::Projection {
        expressions,
        input: Box::new(aggregate),
    }
}

fn build_aggregates(projection: &[ResolvedSelectItem], referenced: &[usize]) -> Vec<AggregateExpr> {
    let mut aggregates = Vec::new();

    for item in projection {
        match &item.expr {
            ResolvedExpr::Column(_) => continue,
            ResolvedExpr::Aggregate(aggregate) => {
                let function = match &aggregate.function {
                    ResolvedAggregateFunction::CountStar => AggregateFunction::CountStar,
                    ResolvedAggregateFunction::Count { column } => AggregateFunction::Count {
                        input: remap_to_scan_index(column.index, referenced),
                    },
                    ResolvedAggregateFunction::Avg { column } => AggregateFunction::Avg {
                        input: remap_to_scan_index(column.index, referenced),
                    },
                };
                aggregates.push(AggregateExpr { function });
            }
        }
    }

    aggregates
}

fn build_group_keys(group_by: &[ResolvedExpr], referenced: &[usize]) -> Vec<ColumnRef> {
    let mut keys = Vec::with_capacity(group_by.len());

    for expression in group_by {
        let ResolvedExpr::Column(column) = expression else {
            panic!("GROUP BY expression must be a column");
        };
        keys.push(remap_to_scan_index(column.index, referenced));
    }

    keys
}

fn build_scan_columns(table: &ResolvedTable, referenced: &[usize]) -> Vec<ScanColumn> {
    referenced
        .iter()
        .map(|&table_index| {
            let column = &table.columns[table_index];
            ScanColumn {
                name: column.name.clone(),
                table_index,
                data_type: column.data_type,
            }
        })
        .collect()
}

fn build_select(
    projection: Vec<ResolvedSelectItem>,
    from: ResolvedTable,
    group_by: Vec<ResolvedExpr>,
) -> LogicalPlan {
    let referenced = collect_referenced_indices(&projection, &group_by);
    let scan_columns = build_scan_columns(&from, &referenced);

    let table_scan = LogicalPlan::TableScan {
        table_id: from.id,
        table_name: from.name,
        columns: scan_columns,
    };

    let has_aggregates = projection.iter().any(|item| is_aggregate(&item.expr));
    let needs_aggregate = has_aggregates || !group_by.is_empty();

    if needs_aggregate {
        build_aggregate_query(projection, group_by, table_scan, &referenced)
    } else {
        build_simple_query(projection, table_scan, &referenced)
    }
}

fn build_simple_query(
    projection: Vec<ResolvedSelectItem>,
    table_scan: LogicalPlan,
    referenced: &[usize],
) -> LogicalPlan {
    let mut expressions = Vec::with_capacity(projection.len());

    for item in projection {
        let ResolvedExpr::Column(column) = item.expr else {
            panic!(
                "non-aggregate query cannot contain \
                 aggregate expressions"
            );
        };
        expressions.push(ProjectionExpr {
            input: remap_to_scan_index(column.index, referenced),
            alias: item.alias,
        });
    }

    LogicalPlan::Projection {
        expressions,
        input: Box::new(table_scan),
    }
}

/// Collects table-level column indices from a resolved
/// expression. Aggregate inputs are always single columns
/// (bind rejects nested aggregates), so we extract directly
/// from the typed function variant.
fn collect_expr_column_indices(expression: &ResolvedExpr, indices: &mut Vec<usize>) {
    match expression {
        ResolvedExpr::Column(column) => {
            indices.push(column.index);
        }
        ResolvedExpr::Aggregate(aggregate) => match &aggregate.function {
            ResolvedAggregateFunction::CountStar => {}
            ResolvedAggregateFunction::Count { column }
            | ResolvedAggregateFunction::Avg { column } => {
                indices.push(column.index);
            }
        },
    }
}

fn collect_referenced_indices(
    projection: &[ResolvedSelectItem],
    group_by: &[ResolvedExpr],
) -> Vec<usize> {
    let mut indices = Vec::new();

    for item in projection {
        collect_expr_column_indices(&item.expr, &mut indices);
    }
    for expression in group_by {
        collect_expr_column_indices(expression, &mut indices);
    }

    indices.sort_unstable();
    indices.dedup();
    indices
}

fn find_group_key_position(column: &ResolvedColumn, group_by: &[ResolvedExpr]) -> usize {
    for (position, expression) in group_by.iter().enumerate() {
        match expression {
            ResolvedExpr::Column(group_column) => {
                if group_column.index == column.index {
                    return position;
                } else {
                    continue;
                }
            }
            ResolvedExpr::Aggregate(_) => continue,
        }
    }
    panic!(
        "column {} (index {}) not found in GROUP BY",
        column.name, column.index
    );
}

fn is_aggregate(expression: &ResolvedExpr) -> bool {
    matches!(expression, ResolvedExpr::Aggregate(_))
}

/// Maps a table-level column index to its position in the
/// scan output. The `referenced` slice is sorted, so the
/// position is the scan output index.
fn remap_to_scan_index(table_index: usize, referenced: &[usize]) -> ColumnRef {
    for (scan_index, &reference_index) in referenced.iter().enumerate() {
        if reference_index == table_index {
            return ColumnRef { index: scan_index };
        } else {
            continue;
        }
    }
    panic!(
        "column table index {table_index} not found in \
         referenced set {referenced:?}"
    );
}
