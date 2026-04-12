use db_catalog::Database;
use db_types::plan::{
    ResolvedAggregate, ResolvedAggregateFunction, ResolvedColumn, ResolvedExpr, ResolvedSelectItem,
    ResolvedTable, UnresolvedExpr, UnresolvedFunctionArgs, UnresolvedSelectItem,
};
use db_types::{DataTypeKind, ResolvedPlan, TableSchema, UnresolvedPlan};

pub fn bind(plan: UnresolvedPlan, database: &Database) -> ResolvedPlan {
    let UnresolvedPlan::Select {
        projection,
        from,
        group_by,
    } = plan;

    bind_select(projection, from, group_by, database)
}

fn assert_unique_column_name(columns: &[ResolvedColumn], column_name: &str) {
    for column in columns {
        if column.name == column_name {
            panic!(
                "duplicate column name in schema: \
                 {column_name}"
            );
        }
    }
}

fn bind_avg(arguments: UnresolvedFunctionArgs, table: &ResolvedTable) -> ResolvedExpr {
    let UnresolvedFunctionArgs::Exprs(arguments) = arguments else {
        panic!("AVG does not support *");
    };
    if arguments.len() != 1 {
        panic!("AVG expects exactly one argument, got {}", arguments.len());
    } else {
        let column = bind_to_column(arguments.into_iter().next().unwrap(), table);
        if !is_numeric_type(column.data_type) {
            panic!("AVG requires numeric argument, got {:?}", column.data_type);
        } else {
            ResolvedExpr::Aggregate(ResolvedAggregate {
                function: ResolvedAggregateFunction::Avg { column },
                data_type: DataTypeKind::F64,
            })
        }
    }
}

fn bind_count(arguments: UnresolvedFunctionArgs, table: &ResolvedTable) -> ResolvedExpr {
    match arguments {
        UnresolvedFunctionArgs::Star => ResolvedExpr::Aggregate(ResolvedAggregate {
            function: ResolvedAggregateFunction::CountStar,
            data_type: DataTypeKind::U64,
        }),
        UnresolvedFunctionArgs::Exprs(arguments) => {
            if arguments.len() != 1 {
                panic!(
                    "COUNT expects exactly one argument, \
                     got {}",
                    arguments.len()
                );
            } else {
                let column = bind_to_column(arguments.into_iter().next().unwrap(), table);
                ResolvedExpr::Aggregate(ResolvedAggregate {
                    function: ResolvedAggregateFunction::Count { column },
                    data_type: DataTypeKind::U64,
                })
            }
        }
    }
}

fn bind_expr(expression: UnresolvedExpr, table: &ResolvedTable) -> ResolvedExpr {
    match expression {
        UnresolvedExpr::Column(name) => ResolvedExpr::Column(resolve_column(&name, table)),
        UnresolvedExpr::Function { name, args } => bind_function(name, args, table),
    }
}

fn bind_function(
    name: String,
    arguments: UnresolvedFunctionArgs,
    table: &ResolvedTable,
) -> ResolvedExpr {
    match name.as_str() {
        "count" => bind_count(arguments, table),
        "avg" => bind_avg(arguments, table),
        _ => {
            panic!(
                "unsupported function during binding: \
                 {name}"
            )
        }
    }
}

fn bind_group_by(group_by: Vec<UnresolvedExpr>, table: &ResolvedTable) -> Vec<ResolvedExpr> {
    let mut bound = Vec::with_capacity(group_by.len());

    for expression in group_by {
        let resolved = bind_expr(expression, table);
        if is_aggregate(&resolved) {
            panic!(
                "GROUP BY cannot contain aggregate \
                 expressions: {resolved:?}"
            );
        } else {
            bound.push(resolved);
        }
    }

    bound
}

fn bind_projection(
    projection: Vec<UnresolvedSelectItem>,
    table: &ResolvedTable,
) -> Vec<ResolvedSelectItem> {
    let mut bound = Vec::with_capacity(projection.len());

    for item in projection {
        bound.push(bind_select_item(item, table));
    }

    bound
}

fn bind_select(
    projection: Vec<UnresolvedSelectItem>,
    from: String,
    group_by: Vec<UnresolvedExpr>,
    database: &Database,
) -> ResolvedPlan {
    let from = resolve_table(&from, database);
    let projection = bind_projection(projection, &from);
    let group_by = bind_group_by(group_by, &from);
    validate_grouped_projection(&projection, &group_by);

    ResolvedPlan::Select {
        projection,
        from,
        group_by,
    }
}

fn bind_select_item(item: UnresolvedSelectItem, table: &ResolvedTable) -> ResolvedSelectItem {
    let UnresolvedSelectItem { expr, alias } = item;

    ResolvedSelectItem {
        expr: bind_expr(expr, table),
        alias,
    }
}

fn bind_to_column(expression: UnresolvedExpr, table: &ResolvedTable) -> ResolvedColumn {
    match expression {
        UnresolvedExpr::Column(name) => resolve_column(&name, table),
        UnresolvedExpr::Function { .. } => {
            panic!(
                "function expressions in aggregate \
                 arguments are unsupported"
            );
        }
    }
}

fn is_aggregate(expression: &ResolvedExpr) -> bool {
    matches!(expression, ResolvedExpr::Aggregate(_))
}

fn is_numeric_type(data_type: DataTypeKind) -> bool {
    match data_type {
        DataTypeKind::U64
        | DataTypeKind::U32
        | DataTypeKind::U8
        | DataTypeKind::I64
        | DataTypeKind::I32
        | DataTypeKind::I8
        | DataTypeKind::F32
        | DataTypeKind::F64 => true,
        DataTypeKind::BOOL => false,
    }
}

fn projection_has_aggregate(projection: &[ResolvedSelectItem]) -> bool {
    projection.iter().any(|item| is_aggregate(&item.expr))
}

fn resolve_column(column_name: &str, table: &ResolvedTable) -> ResolvedColumn {
    for column in &table.columns {
        if column.name == column_name {
            return column.clone();
        }
    }

    panic!("column not found in table {}: {column_name}", table.name);
}

fn resolve_table(table_name: &str, database: &Database) -> ResolvedTable {
    let table = database
        .get_table(table_name)
        .unwrap_or_else(|| panic!("table not found in catalog: {table_name}"));

    ResolvedTable {
        id: table.id(),
        name: table.name().to_owned(),
        columns: resolve_table_columns(table.schema()),
    }
}

fn resolve_table_columns(schema: &TableSchema) -> Vec<ResolvedColumn> {
    let mut columns = Vec::with_capacity(schema.column_count());

    for (index, column) in schema.columns().iter().enumerate() {
        assert_unique_column_name(&columns, column.name());
        columns.push(ResolvedColumn {
            name: column.name().to_owned(),
            index,
            data_type: column.data_type(),
        });
    }

    columns
}

fn validate_aggregate_projection(projection: &[ResolvedSelectItem], group_by: &[ResolvedExpr]) {
    for item in projection {
        let allowed = is_aggregate(&item.expr) || group_by.contains(&item.expr);

        if allowed {
            continue;
        } else {
            panic!(
                "non-aggregate projection must appear \
                 in GROUP BY: {:?}",
                item.expr
            );
        }
    }
}

fn validate_grouped_projection(projection: &[ResolvedSelectItem], group_by: &[ResolvedExpr]) {
    if projection_has_aggregate(projection) {
        validate_aggregate_projection(projection, group_by);
    } else if !group_by.is_empty() {
        validate_non_aggregate_projection(projection, group_by);
    } else {
        // No aggregates and no GROUP BY: plain
        // projection, no validation needed.
    }
}

fn validate_non_aggregate_projection(projection: &[ResolvedSelectItem], group_by: &[ResolvedExpr]) {
    for item in projection {
        if group_by.contains(&item.expr) {
            continue;
        } else {
            panic!(
                "projection must appear in GROUP BY: \
                 {:?}",
                item.expr
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bind, parse, translate};
    use db_catalog::Database;
    use db_types::DataTypeKind;
    use db_types::plan::{
        ResolvedAggregate, ResolvedAggregateFunction, ResolvedExpr, ResolvedPlan,
    };

    fn trips_db() -> Database {
        let mut db = Database::new();
        db.create_table(
            "trips",
            &[
                ("cab_type", "U8"),
                ("passenger_count", "U8"),
                ("total_amount", "F64"),
            ],
        );
        db
    }

    #[test]
    fn bind_group_by_count_star() {
        let plan = bind(
            translate(parse(
                "SELECT cab_type, count(*) \
                 FROM trips GROUP BY cab_type",
            )),
            &trips_db(),
        );
        let ResolvedPlan::Select {
            projection,
            from,
            group_by,
        } = plan;

        assert_eq!(from.id, 0);
        assert_eq!(from.name, "trips");
        assert_eq!(from.columns.len(), 3);
        assert!(matches!(
            &projection[0].expr,
            ResolvedExpr::Column(column)
                if column.name == "cab_type"
                    && column.index == 0
                    && column.data_type == DataTypeKind::U8
        ));
        assert!(matches!(
            &projection[1].expr,
            ResolvedExpr::Aggregate(
                ResolvedAggregate {
                    function:
                        ResolvedAggregateFunction::CountStar,
                    data_type,
                }
            ) if *data_type == DataTypeKind::U64
        ));
        assert!(matches!(
            &group_by[0],
            ResolvedExpr::Column(column)
                if column.name == "cab_type"
                    && column.index == 0
                    && column.data_type == DataTypeKind::U8
        ));
    }

    #[test]
    fn bind_group_by_avg() {
        let plan = bind(
            translate(parse(
                "SELECT passenger_count, \
                 avg(total_amount) FROM trips \
                 GROUP BY passenger_count",
            )),
            &trips_db(),
        );
        let ResolvedPlan::Select {
            projection,
            from,
            group_by,
        } = plan;

        assert_eq!(from.name, "trips");
        assert!(matches!(
            &projection[0].expr,
            ResolvedExpr::Column(column)
                if column.name == "passenger_count"
                    && column.index == 1
                    && column.data_type == DataTypeKind::U8
        ));
        assert!(matches!(
            &projection[1].expr,
            ResolvedExpr::Aggregate(
                ResolvedAggregate {
                    function:
                        ResolvedAggregateFunction::Avg {
                            column,
                        },
                    data_type,
                }
            )
                if *data_type == DataTypeKind::F64
                    && column.name == "total_amount"
                    && column.index == 2
                    && column.data_type
                        == DataTypeKind::F64
        ));
        assert!(matches!(
            &group_by[0],
            ResolvedExpr::Column(column)
                if column.name == "passenger_count"
                    && column.index == 1
                    && column.data_type == DataTypeKind::U8
        ));
    }

    #[test]
    #[should_panic(expected = "AVG requires numeric argument")]
    fn bind_avg_rejects_non_numeric_argument() {
        let mut database = Database::new();
        database.create_table(
            "trips",
            &[("passenger_count", "U8"), ("total_amount", "BOOL")],
        );

        let plan = translate(parse(
            "SELECT passenger_count, \
             avg(total_amount) FROM trips \
             GROUP BY passenger_count",
        ));

        let _ = bind(plan, &database);
    }
}
