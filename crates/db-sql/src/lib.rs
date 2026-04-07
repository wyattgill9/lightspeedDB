use db_catalog::Database;
use db_types::{ResolvedPlan, UnresolvedPlan};

pub struct ParsedStatement(sqlparser::ast::Statement);

pub fn parse(sql: &str) -> ParsedStatement {
    let dialect = sqlparser::dialect::GenericDialect {};

    let mut statements = sqlparser::parser::Parser::parse_sql(&dialect, sql)
        .unwrap_or_else(|error| panic!("failed to parse SQL: {error}"));

    if statements.len() != 1 {
        panic!("expected exactly one statement, got {}", statements.len());
    }

    ParsedStatement(statements.swap_remove(0))
}

pub fn translate(_statement: ParsedStatement) -> UnresolvedPlan {
    UnresolvedPlan::default()
}

pub fn bind(_plan: UnresolvedPlan, _db: &Database) -> ResolvedPlan {
    ResolvedPlan::default()
}
