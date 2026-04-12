use crate::parse::ParsedStatement;
use db_types::UnresolvedPlan;
use db_types::plan::{UnresolvedExpr, UnresolvedFunctionArgs, UnresolvedSelectItem};
use sqlparser::ast::{
    Expr as SqlExpr, FunctionArg, FunctionArgExpr, FunctionArguments, GroupByExpr, SelectFlavor,
    SelectItem as SqlSelectItem, SetExpr, Statement, TableFactor, TableWithJoins,
};

pub fn translate(statement: ParsedStatement) -> UnresolvedPlan {
    let ParsedStatement(statement) = statement;

    match statement {
        Statement::Query(query) => translate_query(*query),
        other => panic!("unsupported statement: {other:?}"),
    }
}

fn ensure_supported_select(select: &sqlparser::ast::Select) {
    if select.optimizer_hint.is_some() {
        panic!("unsupported SELECT optimizer hint");
    } else if select.distinct.is_some() {
        panic!("unsupported SELECT DISTINCT clause");
    } else if select.select_modifiers.is_some() {
        panic!("unsupported SELECT modifiers");
    } else if select.top.is_some() {
        panic!("unsupported SELECT TOP clause");
    } else if select.exclude.is_some() {
        panic!("unsupported SELECT EXCLUDE clause");
    } else if select.into.is_some() {
        panic!("unsupported SELECT INTO clause");
    } else if !select.lateral_views.is_empty() {
        panic!(
            "unsupported SELECT lateral views: {:?}",
            select.lateral_views
        );
    } else if select.prewhere.is_some() {
        panic!("unsupported SELECT PREWHERE clause");
    } else if select.selection.is_some() {
        panic!("unsupported SELECT WHERE clause");
    } else if !select.connect_by.is_empty() {
        panic!("unsupported SELECT CONNECT BY: {:?}", select.connect_by);
    } else if !select.cluster_by.is_empty() {
        panic!("unsupported SELECT CLUSTER BY: {:?}", select.cluster_by);
    } else if !select.distribute_by.is_empty() {
        panic!(
            "unsupported SELECT DISTRIBUTE BY: {:?}",
            select.distribute_by
        );
    } else if !select.sort_by.is_empty() {
        panic!("unsupported SELECT SORT BY: {:?}", select.sort_by);
    } else if select.having.is_some() {
        panic!("unsupported SELECT HAVING clause");
    } else if !select.named_window.is_empty() {
        panic!("unsupported SELECT WINDOW: {:?}", select.named_window);
    } else if select.qualify.is_some() {
        panic!("unsupported SELECT QUALIFY clause");
    } else if select.value_table_mode.is_some() {
        panic!("unsupported SELECT AS VALUE/STRUCT");
    } else if !matches!(select.flavor, SelectFlavor::Standard) {
        panic!("unsupported SELECT flavor: {:?}", select.flavor);
    }
}

fn translate_expr(expression: SqlExpr) -> UnresolvedExpr {
    match expression {
        SqlExpr::Identifier(identifier) => UnresolvedExpr::Column(identifier.value),
        SqlExpr::Function(function) => translate_function(function),
        other => {
            panic!("unsupported expression: {other:?}")
        }
    }
}

fn translate_from(from: Vec<TableWithJoins>) -> String {
    let table = match from.len() {
        1 => match from.into_iter().next() {
            Some(table) => table,
            None => {
                panic!(
                    "expected exactly one table in FROM, \
                     got 0"
                )
            }
        },
        count => {
            panic!(
                "expected exactly one table in FROM, \
                 got {count}"
            )
        }
    };

    let TableWithJoins { relation, joins } = table;

    if joins.is_empty() {
        match relation {
            TableFactor::Table { name, .. } => name.to_string(),
            other => {
                panic!("unsupported table factor: {other:?}")
            }
        }
    } else {
        panic!("unsupported joins: {joins:?}");
    }
}

fn translate_function(function: sqlparser::ast::Function) -> UnresolvedExpr {
    let sqlparser::ast::Function {
        name,
        uses_odbc_syntax,
        parameters,
        args,
        filter,
        null_treatment,
        over,
        within_group,
    } = function;

    if uses_odbc_syntax {
        panic!("unsupported ODBC function syntax");
    } else if !matches!(parameters, FunctionArguments::None) {
        panic!(
            "unsupported function parameters: \
             {parameters:?}"
        );
    } else if filter.is_some() {
        panic!("unsupported function FILTER clause");
    } else if null_treatment.is_some() {
        panic!("unsupported function NULL treatment");
    } else if over.is_some() {
        panic!("unsupported window function");
    } else if !within_group.is_empty() {
        panic!("unsupported WITHIN GROUP: {within_group:?}");
    } else {
        translate_function_arguments(name.to_string().to_lowercase(), args)
    }
}

fn translate_function_arguments(name: String, arguments: FunctionArguments) -> UnresolvedExpr {
    match arguments {
        FunctionArguments::List(argument_list) => {
            let sqlparser::ast::FunctionArgumentList {
                duplicate_treatment,
                args,
                clauses,
            } = argument_list;

            if duplicate_treatment.is_some() {
                panic!(
                    "unsupported function duplicate \
                     treatment"
                );
            } else if !clauses.is_empty() {
                panic!(
                    "unsupported function argument \
                     clauses: {clauses:?}"
                );
            } else if matches!(
                args.as_slice(),
                [FunctionArg::Unnamed(FunctionArgExpr::Wildcard)]
            ) {
                UnresolvedExpr::Function {
                    name,
                    args: UnresolvedFunctionArgs::Star,
                }
            } else {
                let mut translated = Vec::with_capacity(args.len());

                for argument in args {
                    match argument {
                        FunctionArg::Unnamed(FunctionArgExpr::Expr(expression)) => {
                            translated.push(translate_expr(expression));
                        }
                        other => {
                            panic!(
                                "unsupported function \
                                 argument: {other:?}"
                            )
                        }
                    }
                }

                UnresolvedExpr::Function {
                    name,
                    args: UnresolvedFunctionArgs::Exprs(translated),
                }
            }
        }
        other => {
            panic!("unsupported function arguments: {other:?}")
        }
    }
}

fn translate_group_by(group_by: GroupByExpr) -> Vec<UnresolvedExpr> {
    match group_by {
        GroupByExpr::Expressions(expressions, modifiers) => {
            if modifiers.is_empty() {
                expressions.into_iter().map(translate_expr).collect()
            } else {
                panic!(
                    "unsupported GROUP BY modifiers: \
                     {modifiers:?}"
                );
            }
        }
        other => {
            panic!(
                "unsupported GROUP BY expression: \
                 {other:?}"
            )
        }
    }
}

fn translate_query(query: sqlparser::ast::Query) -> UnresolvedPlan {
    let sqlparser::ast::Query {
        with,
        body,
        order_by,
        limit_clause,
        fetch,
        locks,
        for_clause,
        settings,
        format_clause,
        pipe_operators,
    } = query;

    if with.is_some() {
        panic!("unsupported query WITH clause");
    } else if order_by.is_some() {
        panic!("unsupported query ORDER BY clause");
    } else if limit_clause.is_some() {
        panic!("unsupported query LIMIT clause");
    } else if fetch.is_some() {
        panic!("unsupported query FETCH clause");
    } else if !locks.is_empty() {
        panic!("unsupported query locks: {locks:?}");
    } else if for_clause.is_some() {
        panic!("unsupported query FOR clause");
    } else if settings.is_some() {
        panic!("unsupported query SETTINGS clause");
    } else if format_clause.is_some() {
        panic!("unsupported query FORMAT clause");
    } else if !pipe_operators.is_empty() {
        panic!(
            "unsupported query pipe operators: \
             {pipe_operators:?}"
        );
    } else {
        match *body {
            SetExpr::Select(select) => translate_select(*select),
            other => {
                panic!("unsupported query body: {other:?}")
            }
        }
    }
}

fn translate_select(select: sqlparser::ast::Select) -> UnresolvedPlan {
    ensure_supported_select(&select);

    let sqlparser::ast::Select {
        projection,
        from,
        group_by,
        ..
    } = select;
    let projection = projection.into_iter().map(translate_select_item).collect();
    let from = translate_from(from);
    let group_by = translate_group_by(group_by);

    UnresolvedPlan::Select {
        projection,
        from,
        group_by,
    }
}

fn translate_select_item(item: SqlSelectItem) -> UnresolvedSelectItem {
    match item {
        SqlSelectItem::UnnamedExpr(expression) => UnresolvedSelectItem {
            expr: translate_expr(expression),
            alias: None,
        },
        SqlSelectItem::ExprWithAlias {
            expr: expression,
            alias,
        } => UnresolvedSelectItem {
            expr: translate_expr(expression),
            alias: Some(alias.value),
        },
        other => {
            panic!("unsupported select item: {other:?}")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, translate};
    use db_types::UnresolvedPlan;
    use db_types::plan::{UnresolvedExpr, UnresolvedFunctionArgs};

    #[test]
    fn query_group_by_count_star() {
        let sql = "SELECT cab_type, count(*) \
                   FROM trips GROUP BY cab_type";
        let plan = translate(parse(sql));
        let UnresolvedPlan::Select {
            projection,
            from,
            group_by,
        } = plan;

        assert_eq!(projection.len(), 2);
        assert!(matches!(
            &projection[0].expr,
            UnresolvedExpr::Column(column)
                if column == "cab_type"
        ));
        assert!(projection[0].alias.is_none());
        assert!(matches!(
            &projection[1].expr,
            UnresolvedExpr::Function {
                name,
                args: UnresolvedFunctionArgs::Star,
            } if name == "count"
        ));
        assert_eq!(from, "trips");
        assert_eq!(group_by.len(), 1);
        assert!(matches!(
            &group_by[0],
            UnresolvedExpr::Column(column)
                if column == "cab_type"
        ));
    }

    #[test]
    fn query_group_by_avg() {
        let sql = "SELECT passenger_count, \
                   avg(total_amount) FROM trips \
                   GROUP BY passenger_count";
        let plan = translate(parse(sql));
        let UnresolvedPlan::Select {
            projection,
            from,
            group_by,
        } = plan;

        assert_eq!(projection.len(), 2);
        if let UnresolvedExpr::Function {
            name,
            args: UnresolvedFunctionArgs::Exprs(arguments),
        } = &projection[1].expr
        {
            assert_eq!(name, "avg");
            assert_eq!(arguments.len(), 1);
            assert!(matches!(
                &arguments[0],
                UnresolvedExpr::Column(column)
                    if column == "total_amount"
            ));
        } else {
            panic!("expected avg function");
        }
        assert_eq!(from, "trips");
        assert_eq!(group_by.len(), 1);
        assert!(matches!(
            &group_by[0],
            UnresolvedExpr::Column(column)
                if column == "passenger_count"
        ));
    }
}
