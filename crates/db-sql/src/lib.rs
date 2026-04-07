struct ParsedStatement(pub(crate) sqlparser::ast::Statement);

/// Parse and bind a SQL statement against the catalog.
///
/// Returns a LogicalPlan with all names resolved to indices.
fn parse(sql: &str) -> ParsedStatement {
    let dialect = sqlparser::dialect::GenericDialect {};

    let mut statements = sqlparser::parser::Parser::parse_sql(&dialect, sql)
        .unwrap_or_else(|error| panic!("failed to parse SQL: {error}"));

    if statements.len() != 1 {
        panic!("expected exactly one statement, got {}", statements.len());
    }

    ParsedStatement(statements.swap_remove(0))
}

    // let sqlparser::ast::Statement::Query(query) = statement else {
    //     panic!("only SELECT queries are supported");
    // };

    // let sqlparser::ast::SetExpr::Select(select) = *query.body else {
    //     panic!("only simple SELECT statements are supported");
    // };


    // let table = database
    //     .get_table(&table_name)
    //     .unwrap_or_else(|| panic!("table not found: {table_name}"));

    // LogicalPlan::Scan {
    //     table_name,
    //     column_indices,
    // }
// }

// fn bind_from_clause(from: &[sqlparser::ast::TableWithJoins]) -> String {
//     if from.len() != 1 {
//         panic!("expected exactly one table in FROM, got {}", from.len());
//     }

//     let table_with_joins = &from[0];

//     if !table_with_joins.joins.is_empty() {
//         panic!("JOIN is not supported yet");
//     }

//     let sqlparser::ast::TableFactor::Table { name, .. } = &table_with_joins.relation else {
//         panic!("only simple table references are supported");
//     };

//     format!("{}", name)
// }

// fn bind_projection(
//     projection: &[sqlparser::ast::SelectItem],
//     schema: &db_types::TableSchema,
// ) -> Vec<usize> {
//     match projection {
//         [sqlparser::ast::SelectItem::Wildcard(_)] => (0..schema.column_count()).collect(),
//         _ => panic!("only SELECT * is supported yet"),
//     }
// }
