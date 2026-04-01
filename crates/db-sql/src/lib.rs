pub fn parse(query: &str) -> String {
    let dialect = sqlparser::dialect::GenericDialect {}; // or AnsiDialect
    let ast = sqlparser::parser::Parser::parse_sql(&dialect, query).unwrap();
    format!("{:?}", ast)
}
