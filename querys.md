```rs
"SELECT col1, col2 FROM table1"
"SELECT * FROM table1"
"SELECT col1 FROM table1 WHERE col1 = 'value'"
"SELECT col1 FROM table1 WHERE col1 > 10 AND col2 < 100"
"SELECT col1 FROM table1 WHERE col1 = 1 OR col2 = 2"
"SELECT COUNT(*) FROM table1"
"SELECT col1, COUNT(*) FROM table1 GROUP BY col1"
"SELECT col1, SUM(col2), AVG(col3) FROM table1 GROUP BY col1"
"SELECT col1, MIN(col2), MAX(col2) FROM table1 GROUP BY col1"
"SELECT col1 FROM table1 ORDER BY col1 ASC"
"SELECT col1 FROM table1 ORDER BY col1 DESC"
"SELECT col1 FROM table1 LIMIT 10"
"SELECT col1 FROM table1 LIMIT 10 OFFSET 20"
"SELECT col1, COUNT(*) FROM table1 GROUP BY col1 HAVING COUNT(*) > 5"
"SELECT col1, col2 * col3 AS revenue FROM table1"
"SELECT COUNT(DISTINCT col1) FROM table1"
"SELECT col1 FROM table1 WHERE col1 IN ('a', 'b', 'c')"
```

```bash
Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col2",
                                quote_style: None,
                                span: Span(Location(1,14)..Location(1,18)),
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,24)..Location(1,30)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)



Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    Wildcard(
                        WildcardAdditionalOptions {
                            wildcard_token: TokenWithSpan {
                                token: Mul,
                                span: Span(Location(1,8)..Location(1,9)),
                            },
                            opt_ilike: None,
                            opt_exclude: None,
                            opt_except: None,
                            opt_replace: None,
                            opt_rename: None,
                        },
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,15)..Location(1,21)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,18)..Location(1,24)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: Some(
                    BinaryOp {
                        left: Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,31)..Location(1,35)),
                            },
                        ),
                        op: Eq,
                        right: Value(
                            ValueWithSpan {
                                value: SingleQuotedString(
                                    "value",
                                ),
                                span: Span(Location(1,38)..Location(1,45)),
                            },
                        ),
                    },
                ),
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,18)..Location(1,24)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: Some(
                    BinaryOp {
                        left: BinaryOp {
                            left: Identifier(
                                Ident {
                                    value: "col1",
                                    quote_style: None,
                                    span: Span(Location(1,31)..Location(1,35)),
                                },
                            ),
                            op: Gt,
                            right: Value(
                                ValueWithSpan {
                                    value: Number(
                                        "10",
                                        false,
                                    ),
                                    span: Span(Location(1,38)..Location(1,40)),
                                },
                            ),
                        },
                        op: And,
                        right: BinaryOp {
                            left: Identifier(
                                Ident {
                                    value: "col2",
                                    quote_style: None,
                                    span: Span(Location(1,45)..Location(1,49)),
                                },
                            ),
                            op: Lt,
                            right: Value(
                                ValueWithSpan {
                                    value: Number(
                                        "100",
                                        false,
                                    ),
                                    span: Span(Location(1,52)..Location(1,55)),
                                },
                            ),
                        },
                    },
                ),
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,18)..Location(1,24)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: Some(
                    BinaryOp {
                        left: BinaryOp {
                            left: Identifier(
                                Ident {
                                    value: "col1",
                                    quote_style: None,
                                    span: Span(Location(1,31)..Location(1,35)),
                                },
                            ),
                            op: Eq,
                            right: Value(
                                ValueWithSpan {
                                    value: Number(
                                        "1",
                                        false,
                                    ),
                                    span: Span(Location(1,38)..Location(1,39)),
                                },
                            ),
                        },
                        op: Or,
                        right: BinaryOp {
                            left: Identifier(
                                Ident {
                                    value: "col2",
                                    quote_style: None,
                                    span: Span(Location(1,43)..Location(1,47)),
                                },
                            ),
                            op: Eq,
                            right: Value(
                                ValueWithSpan {
                                    value: Number(
                                        "2",
                                        false,
                                    ),
                                    span: Span(Location(1,50)..Location(1,51)),
                                },
                            ),
                        },
                    },
                ),
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Function(
                            Function {
                                name: ObjectName(
                                    [
                                        Identifier(
                                            Ident {
                                                value: "COUNT",
                                                quote_style: None,
                                                span: Span(Location(1,8)..Location(1,13)),
                                            },
                                        ),
                                    ],
                                ),
                                uses_odbc_syntax: false,
                                parameters: None,
                                args: List(
                                    FunctionArgumentList {
                                        duplicate_treatment: None,
                                        args: [
                                            Unnamed(
                                                Wildcard,
                                            ),
                                        ],
                                        clauses: [],
                                    },
                                ),
                                filter: None,
                                null_treatment: None,
                                over: None,
                                within_group: [],
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,22)..Location(1,28)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                    UnnamedExpr(
                        Function(
                            Function {
                                name: ObjectName(
                                    [
                                        Identifier(
                                            Ident {
                                                value: "COUNT",
                                                quote_style: None,
                                                span: Span(Location(1,14)..Location(1,19)),
                                            },
                                        ),
                                    ],
                                ),
                                uses_odbc_syntax: false,
                                parameters: None,
                                args: List(
                                    FunctionArgumentList {
                                        duplicate_treatment: None,
                                        args: [
                                            Unnamed(
                                                Wildcard,
                                            ),
                                        ],
                                        clauses: [],
                                    },
                                ),
                                filter: None,
                                null_treatment: None,
                                over: None,
                                within_group: [],
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,28)..Location(1,34)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,44)..Location(1,48)),
                            },
                        ),
                    ],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                    UnnamedExpr(
                        Function(
                            Function {
                                name: ObjectName(
                                    [
                                        Identifier(
                                            Ident {
                                                value: "SUM",
                                                quote_style: None,
                                                span: Span(Location(1,14)..Location(1,17)),
                                            },
                                        ),
                                    ],
                                ),
                                uses_odbc_syntax: false,
                                parameters: None,
                                args: List(
                                    FunctionArgumentList {
                                        duplicate_treatment: None,
                                        args: [
                                            Unnamed(
                                                Expr(
                                                    Identifier(
                                                        Ident {
                                                            value: "col2",
                                                            quote_style: None,
                                                            span: Span(Location(1,18)..Location(1,22)),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                        clauses: [],
                                    },
                                ),
                                filter: None,
                                null_treatment: None,
                                over: None,
                                within_group: [],
                            },
                        ),
                    ),
                    UnnamedExpr(
                        Function(
                            Function {
                                name: ObjectName(
                                    [
                                        Identifier(
                                            Ident {
                                                value: "AVG",
                                                quote_style: None,
                                                span: Span(Location(1,25)..Location(1,28)),
                                            },
                                        ),
                                    ],
                                ),
                                uses_odbc_syntax: false,
                                parameters: None,
                                args: List(
                                    FunctionArgumentList {
                                        duplicate_treatment: None,
                                        args: [
                                            Unnamed(
                                                Expr(
                                                    Identifier(
                                                        Ident {
                                                            value: "col3",
                                                            quote_style: None,
                                                            span: Span(Location(1,29)..Location(1,33)),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                        clauses: [],
                                    },
                                ),
                                filter: None,
                                null_treatment: None,
                                over: None,
                                within_group: [],
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,40)..Location(1,46)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,56)..Location(1,60)),
                            },
                        ),
                    ],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                    UnnamedExpr(
                        Function(
                            Function {
                                name: ObjectName(
                                    [
                                        Identifier(
                                            Ident {
                                                value: "MIN",
                                                quote_style: None,
                                                span: Span(Location(1,14)..Location(1,17)),
                                            },
                                        ),
                                    ],
                                ),
                                uses_odbc_syntax: false,
                                parameters: None,
                                args: List(
                                    FunctionArgumentList {
                                        duplicate_treatment: None,
                                        args: [
                                            Unnamed(
                                                Expr(
                                                    Identifier(
                                                        Ident {
                                                            value: "col2",
                                                            quote_style: None,
                                                            span: Span(Location(1,18)..Location(1,22)),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                        clauses: [],
                                    },
                                ),
                                filter: None,
                                null_treatment: None,
                                over: None,
                                within_group: [],
                            },
                        ),
                    ),
                    UnnamedExpr(
                        Function(
                            Function {
                                name: ObjectName(
                                    [
                                        Identifier(
                                            Ident {
                                                value: "MAX",
                                                quote_style: None,
                                                span: Span(Location(1,25)..Location(1,28)),
                                            },
                                        ),
                                    ],
                                ),
                                uses_odbc_syntax: false,
                                parameters: None,
                                args: List(
                                    FunctionArgumentList {
                                        duplicate_treatment: None,
                                        args: [
                                            Unnamed(
                                                Expr(
                                                    Identifier(
                                                        Ident {
                                                            value: "col2",
                                                            quote_style: None,
                                                            span: Span(Location(1,29)..Location(1,33)),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                        clauses: [],
                                    },
                                ),
                                filter: None,
                                null_treatment: None,
                                over: None,
                                within_group: [],
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,40)..Location(1,46)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,56)..Location(1,60)),
                            },
                        ),
                    ],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,18)..Location(1,24)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: Some(
            OrderBy {
                kind: Expressions(
                    [
                        OrderByExpr {
                            expr: Identifier(
                                Ident {
                                    value: "col1",
                                    quote_style: None,
                                    span: Span(Location(1,34)..Location(1,38)),
                                },
                            ),
                            options: OrderByOptions {
                                asc: Some(
                                    true,
                                ),
                                nulls_first: None,
                            },
                            with_fill: None,
                        },
                    ],
                ),
                interpolate: None,
            },
        ),
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,18)..Location(1,24)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: Some(
            OrderBy {
                kind: Expressions(
                    [
                        OrderByExpr {
                            expr: Identifier(
                                Ident {
                                    value: "col1",
                                    quote_style: None,
                                    span: Span(Location(1,34)..Location(1,38)),
                                },
                            ),
                            options: OrderByOptions {
                                asc: Some(
                                    false,
                                ),
                                nulls_first: None,
                            },
                            with_fill: None,
                        },
                    ],
                ),
                interpolate: None,
            },
        ),
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,18)..Location(1,24)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: Some(
            LimitOffset {
                limit: Some(
                    Value(
                        ValueWithSpan {
                            value: Number(
                                "10",
                                false,
                            ),
                            span: Span(Location(1,31)..Location(1,33)),
                        },
                    ),
                ),
                offset: None,
                limit_by: [],
            },
        ),
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,18)..Location(1,24)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: Some(
            LimitOffset {
                limit: Some(
                    Value(
                        ValueWithSpan {
                            value: Number(
                                "10",
                                false,
                            ),
                            span: Span(Location(1,31)..Location(1,33)),
                        },
                    ),
                ),
                offset: Some(
                    Offset {
                        value: Value(
                            ValueWithSpan {
                                value: Number(
                                    "20",
                                    false,
                                ),
                                span: Span(Location(1,41)..Location(1,43)),
                            },
                        ),
                        rows: None,
                    },
                ),
                limit_by: [],
            },
        ),
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                    UnnamedExpr(
                        Function(
                            Function {
                                name: ObjectName(
                                    [
                                        Identifier(
                                            Ident {
                                                value: "COUNT",
                                                quote_style: None,
                                                span: Span(Location(1,14)..Location(1,19)),
                                            },
                                        ),
                                    ],
                                ),
                                uses_odbc_syntax: false,
                                parameters: None,
                                args: List(
                                    FunctionArgumentList {
                                        duplicate_treatment: None,
                                        args: [
                                            Unnamed(
                                                Wildcard,
                                            ),
                                        ],
                                        clauses: [],
                                    },
                                ),
                                filter: None,
                                null_treatment: None,
                                over: None,
                                within_group: [],
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,28)..Location(1,34)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,44)..Location(1,48)),
                            },
                        ),
                    ],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: Some(
                    BinaryOp {
                        left: Function(
                            Function {
                                name: ObjectName(
                                    [
                                        Identifier(
                                            Ident {
                                                value: "COUNT",
                                                quote_style: None,
                                                span: Span(Location(1,56)..Location(1,61)),
                                            },
                                        ),
                                    ],
                                ),
                                uses_odbc_syntax: false,
                                parameters: None,
                                args: List(
                                    FunctionArgumentList {
                                        duplicate_treatment: None,
                                        args: [
                                            Unnamed(
                                                Wildcard,
                                            ),
                                        ],
                                        clauses: [],
                                    },
                                ),
                                filter: None,
                                null_treatment: None,
                                over: None,
                                within_group: [],
                            },
                        ),
                        op: Gt,
                        right: Value(
                            ValueWithSpan {
                                value: Number(
                                    "5",
                                    false,
                                ),
                                span: Span(Location(1,67)..Location(1,68)),
                            },
                        ),
                    },
                ),
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                    ExprWithAlias {
                        expr: BinaryOp {
                            left: Identifier(
                                Ident {
                                    value: "col2",
                                    quote_style: None,
                                    span: Span(Location(1,14)..Location(1,18)),
                                },
                            ),
                            op: Multiply,
                            right: Identifier(
                                Ident {
                                    value: "col3",
                                    quote_style: None,
                                    span: Span(Location(1,21)..Location(1,25)),
                                },
                            ),
                        },
                        alias: Ident {
                            value: "revenue",
                            quote_style: None,
                            span: Span(Location(1,29)..Location(1,36)),
                        },
                    },
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,42)..Location(1,48)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Function(
                            Function {
                                name: ObjectName(
                                    [
                                        Identifier(
                                            Ident {
                                                value: "COUNT",
                                                quote_style: None,
                                                span: Span(Location(1,8)..Location(1,13)),
                                            },
                                        ),
                                    ],
                                ),
                                uses_odbc_syntax: false,
                                parameters: None,
                                args: List(
                                    FunctionArgumentList {
                                        duplicate_treatment: Some(
                                            Distinct,
                                        ),
                                        args: [
                                            Unnamed(
                                                Expr(
                                                    Identifier(
                                                        Ident {
                                                            value: "col1",
                                                            quote_style: None,
                                                            span: Span(Location(1,23)..Location(1,27)),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                        clauses: [],
                                    },
                                ),
                                filter: None,
                                null_treatment: None,
                                over: None,
                                within_group: [],
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,34)..Location(1,40)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: None,
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)


Query(
    Query {
        with: None,
        body: Select(
            Select {
                select_token: TokenWithSpan {
                    token: Word(
                        Word {
                            value: "SELECT",
                            quote_style: None,
                            keyword: SELECT,
                        },
                    ),
                    span: Span(Location(1,1)..Location(1,7)),
                },
                optimizer_hint: None,
                distinct: None,
                select_modifiers: None,
                top: None,
                top_before_distinct: false,
                projection: [
                    UnnamedExpr(
                        Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,8)..Location(1,12)),
                            },
                        ),
                    ),
                ],
                exclude: None,
                into: None,
                from: [
                    TableWithJoins {
                        relation: Table {
                            name: ObjectName(
                                [
                                    Identifier(
                                        Ident {
                                            value: "table1",
                                            quote_style: None,
                                            span: Span(Location(1,18)..Location(1,24)),
                                        },
                                    ),
                                ],
                            ),
                            alias: None,
                            args: None,
                            with_hints: [],
                            version: None,
                            with_ordinality: false,
                            partitions: [],
                            json_path: None,
                            sample: None,
                            index_hints: [],
                        },
                        joins: [],
                    },
                ],
                lateral_views: [],
                prewhere: None,
                selection: Some(
                    InList {
                        expr: Identifier(
                            Ident {
                                value: "col1",
                                quote_style: None,
                                span: Span(Location(1,31)..Location(1,35)),
                            },
                        ),
                        list: [
                            Value(
                                ValueWithSpan {
                                    value: SingleQuotedString(
                                        "a",
                                    ),
                                    span: Span(Location(1,40)..Location(1,43)),
                                },
                            ),
                            Value(
                                ValueWithSpan {
                                    value: SingleQuotedString(
                                        "b",
                                    ),
                                    span: Span(Location(1,45)..Location(1,48)),
                                },
                            ),
                            Value(
                                ValueWithSpan {
                                    value: SingleQuotedString(
                                        "c",
                                    ),
                                    span: Span(Location(1,50)..Location(1,53)),
                                },
                            ),
                        ],
                        negated: false,
                    },
                ),
                connect_by: [],
                group_by: Expressions(
                    [],
                    [],
                ),
                cluster_by: [],
                distribute_by: [],
                sort_by: [],
                having: None,
                named_window: [],
                qualify: None,
                window_before_qualify: false,
                value_table_mode: None,
                flavor: Standard,
            },
        ),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: [],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: [],
    },
)
```
