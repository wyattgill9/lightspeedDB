# Current Architecture

This repository is a Rust 2024 workspace for an analytical database prototype.
The implemented layers are: in-memory columnar storage, catalog, and SQL front-end
(parse, translate, bind). Logical planning, physical planning, and execution are
all stubs.

## Repo Status

- The workspace default member is `crates/db-cli`.
- `cargo clippy --workspace --all-targets` succeeds with only pre-existing
  warnings in `db-server` (unused function), `db-catalog` bench (digit
  grouping), and `db-execution` bench (clone on copy).
- `cargo nextest run --workspace` passes with 5 tests (all in `db-sql`).
- `cargo run -p db-cli` creates a table, runs SQL through parse → translate →
  bind, and prints the `ResolvedPlan`. Logical planning is commented out.
- There is no external error handling crate. All error paths use `panic!`.

## Workspace

```text
db/
├── crates/
│   ├── db-catalog
│   ├── db-cli
│   ├── db-execution
│   ├── db-mvcc
│   ├── db-optimizer
│   ├── db-scheduler
│   ├── db-server
│   ├── db-sql
│   ├── db-storage
│   ├── db-types
│   └── db-wal
├── Cargo.toml
├── Justfile
├── CLAUDE.md
└── CURRENT_ARCH.md
```

## What Works Today

### Implemented write/storage path

```text
typed rows
  -> bytemuck byte cast
  -> db_catalog::Database::insert()
  -> db_catalog::DBTable write_buffer
  -> db_catalog::DBTable::flush_write_buffer()
  -> db_storage::TablePartition::insert_rows()
  -> db_storage::ColumnSegment
```

This path is real and writes row-major input into in-memory columnar storage.

### Implemented query pipeline

```text
SQL text
  -> db_sql::parse()      -- sqlparser wrapper, validates single statement
  -> db_sql::translate()  -- AST -> UnresolvedPlan (SELECT only)
  -> db_sql::bind()       -- resolves tables/columns/functions against catalog
  -> ResolvedPlan
```

`parse()` wraps `sqlparser` and requires exactly one statement.

`translate()` converts the parsed AST into an `UnresolvedPlan`. It supports
`SELECT` with projections and `GROUP BY`. It explicitly rejects unsupported
syntax: `WHERE`, `ORDER BY`, `LIMIT`, `DISTINCT`, `JOIN`, window functions,
CTEs, and subqueries.

`bind()` resolves table and column references against the catalog, validates
aggregate semantics (GROUP BY rules, type checking for `avg`), and produces a
`ResolvedPlan` with typed aggregate function variants, resolved column indices,
and types. Supported functions: `count(*)`, `count(column)`, `avg(column)`.

The pipeline stops at `ResolvedPlan`. The `build_plan` and `optimize` calls in
`db-cli` are commented out.

### Stub planning and execution path

```text
ResolvedPlan
  -> db_optimizer::build_plan()   -- ignores input, returns LogicalPlan::None
  -> db_optimizer::optimize()     -- pass-through
  -> db_execution::physical_plan() -- returns PhysicalPlan::None
  -> db_execution::execute()       -- returns QueryResult::default()
```

All four steps are no-op stubs.

## Crate Responsibilities

### `db-types`

Shared schema, datatype, plan, and result types.

- `DataTypeKind` supports `U64`, `U32`, `U8`, `I64`, `I32`, `I8`, `F32`,
  `F64`, and `BOOL`.
- `ColumnDefinition` stores column name, type, and a cached width.
- `TableSchema` stores ordered columns and precomputes `row_size_bytes`.

Plan types form a staged pipeline with four distinct representations:

- `UnresolvedPlan` — enum with `Select` variant holding projection
  (`Vec<UnresolvedSelectItem>`), table name (`String`), and `group_by`
  (`Vec<UnresolvedExpr>`). `UnresolvedExpr` covers column references
  (`Column(String)`) and functions (`Function { name, args }`).
  `UnresolvedFunctionArgs` distinguishes `Star` from `Exprs(Vec<UnresolvedExpr>)`.

- `ResolvedPlan` — enum with `Select` variant holding resolved projection
  (`Vec<ResolvedSelectItem>`), `ResolvedTable` (id, name, columns), and
  resolved `group_by` (`Vec<ResolvedExpr>`). `ResolvedExpr` is either
  `Column(ResolvedColumn)` or `Aggregate(ResolvedAggregate)`.
  `ResolvedAggregate` carries a typed `ResolvedAggregateFunction` enum
  (`CountStar`, `Count { column }`, `Avg { column }`) and a `data_type`.
  No stringly-typed function dispatch.

- `LogicalPlan` — enum with only a `None` variant (stub).

- `PhysicalPlan` — enum with only a `None` variant (stub).

- `QueryResult` is an empty struct (placeholder).
- `OutputTable` wraps a `String`; `from_query_result()` returns an empty
  output string.

### `db-storage`

Implemented in-memory columnar storage primitives.

- `TablePartition` owns one `ColumnSegment` per schema column plus `row_count`.
- `TablePartition::insert_rows()` transposes tightly packed row-major input into
  column-major segment buffers.
- Partition capacity is fixed at `64 * 2048 = 131_072` rows.
- `ColumnSegment` stores:
  - dense `Vec<u8>` column data
  - source column index
  - `ZoneMap`
  - optional Bloom filter
  - HyperLogLog cardinality estimator
- `ZoneMap` stores fixed-width min/max bytes for supported primitive types.
- `varlen.rs` contains an unfinished arena-backed string sketch. The module is
  compiled but all functions are `todo!()`.

### `db-catalog`

Implemented in-memory catalog and table lifecycle layer.

- `Database` stores tables in a
  `HashMap<String, DBTable, rapidhash::fast::RandomState>`.
- `create_table_with_schema()` assigns table ids from the current table count.
- `create_table()` builds a schema from `(&str, &str)` field pairs.
- `insert()` appends bytes into a table-local write buffer.
- `flush_table_writes()` drains the write buffer into partitions.
- `table()` / `table_mut()` panic if the table does not exist.
- `get_table()` is the only non-panicking lookup path.

`DBTable` owns:

- `meta: TableMeta { name, id }`
- `schema: TableSchema`
- `table_partitions: Vec<TablePartition>`
- `stats: TableStatistics`
- `write_buffer: Vec<u8>`

`TableStatistics` currently exists but has no fields.

### `db-sql`

SQL front-end with real parsing, translation, and binding.

- `parse(sql)` uses `sqlparser::parser::Parser::parse_sql`. Panics on parse
  failure or if the input contains anything other than exactly one statement.
- `translate(ParsedStatement)` converts the AST into an `UnresolvedPlan`.
  Supports `SELECT` projections (columns, `count(*)`, `count(col)`,
  `avg(col)`) and `GROUP BY`. Panics on unsupported syntax.
- `bind(UnresolvedPlan, &Database)` resolves table and column references against
  the catalog. Validates aggregate semantics: columns in projections must appear
  in `GROUP BY` unless inside an aggregate, `avg` requires a numeric column.
  Produces `ResolvedPlan` with typed `ResolvedAggregateFunction` variants.
  Panics on semantic errors.

5 tests cover the parse-translate-bind pipeline for GROUP BY queries with
`count(*)` and `avg()`, plus a rejection test for `avg` on non-numeric columns.

### `db-optimizer`

Stub logical plan construction and optimization.

- `build_plan(ResolvedPlan) -> LogicalPlan` ignores its input and returns
  `LogicalPlan::None`. The full implementation was written and then removed;
  this is the re-stub state ahead of the next iteration.
- `optimize(LogicalPlan) -> LogicalPlan` is a pass-through. This is the
  insertion point for future optimization passes (predicate pushdown, join
  reordering, etc.).

### `db-execution`

Physical planning and execution APIs exist, but they do not use storage.

- `physical_plan()` ignores `LogicalPlan` and returns `PhysicalPlan::None`.
- `execute()` ignores both the physical plan and the `Database`, and returns
  `QueryResult::default()`.

The execution benchmark (`benches/query.rs`) loads 100K `Vec3` rows and
benchmarks `table_scan` execution. It compiles and runs but exercises no real
execution logic since execution is stubbed.

### `db-cli`

The current demo binary wires the crates together:

1. creates a `trips` table with columns `(cab_type: u8, passenger_count: u8,
   total_amount: f64)`
2. runs `SELECT passenger_count, avg(total_amount) FROM trips GROUP BY
   passenger_count` through parse → translate → bind
3. prints the `Debug` form of the `ResolvedPlan`

The `build_plan` / `optimize` calls are commented out. The execution layer is
not invoked.

The binary's `main()` is compiled only on 64-bit targets.

### `db-server`

Networking scaffold only.

- Contains a private `run_server_main()` async function (currently unused).
- Binds `0.0.0.0:8080`, converts the socket into a Tokio `TcpListener`, and
  accepts one connection.
- No protocol handling and no integration with the database crates.

### `db-mvcc`, `db-scheduler`, `db-wal`

These crates have manifests and dependency edges, but their `lib.rs` files are
empty.

## Storage Model

The storage layer is columnar after flush, but ingestion is row-major bytes.

```text
Database
└── HashMap<String, DBTable, rapidhash::fast::RandomState>
    └── DBTable
        ├── meta: TableMeta
        ├── schema: TableSchema
        ├── table_partitions: Vec<TablePartition>
        ├── stats: TableStatistics
        └── write_buffer: Vec<u8>
```

### Write Buffer

- Input to `DBTable::insert()` must be a multiple of `schema.row_size_bytes()`.
- Rows accumulate in `write_buffer`.
- The buffer auto-flushes once it reaches `4096 * row_size_bytes()` bytes.
- `flush_write_buffer()` swaps the buffer out, writes it into partitions, then
  restores the cleared allocation for reuse.

### Partitions

- Each `TablePartition` starts with one `ColumnSegment` per schema column.
- Partition capacity is fixed at `64 * 2048 = 131_072` rows.
- When the current partition is full, `DBTable` allocates a new partition and
  continues writing remaining rows.

### Column Segments

Each `ColumnSegment` is append-only today.

For every inserted value it:

- appends raw bytes to the dense column buffer
- updates the Bloom filter when present
- updates the HyperLogLog estimator
- updates the zone map using the column datatype

The storage crate builds metadata on write, but none of that metadata is
consulted by the current query path.

## Dependency Shape

```text
db-types
  -> db-storage
  -> db-catalog

db-sql -> { db-types, db-catalog }
db-optimizer -> { db-types }
db-execution -> { db-types, db-storage, db-catalog }
db-cli -> {
  db-types,
  db-storage,
  db-catalog,
  db-sql,
  db-optimizer,
  db-execution
}

db-mvcc -> { db-types, db-storage }
db-scheduler -> { db-types, db-execution }
db-wal -> { db-types, db-storage }
db-server -> { tokio, socket2 }
```

## Build and Benchmark Notes

- `Justfile` defines `build`, `run`, `bench`, `check`, `test`, and `fmt`
  shortcuts.
- `crates/db-catalog/benches/insert.rs` matches the current storage/catalog
  APIs and builds.
- `crates/db-execution/benches/query.rs` compiles and runs but exercises no
  real execution logic since the execution layer is stubbed.
- The top-level `build.rs` requires a 64-bit target, but the workspace root is
  not a package, so that `build.rs` is currently inactive.

## Current Gaps

The largest gaps between the current repository and a functioning database
engine are:

- no durable storage
- no WAL integration
- no MVCC implementation
- no scheduler implementation
- no logical planning (LogicalPlan is a stub enum with only `None`)
- no physical planning (PhysicalPlan is a stub enum with only `None`)
- no execution engine over stored partitions
- no typed error model; all failure paths panic
- SQL support limited to SELECT with GROUP BY (no WHERE, ORDER BY, LIMIT, JOIN)
- `OutputTable` and `QueryResult` are still placeholders
- no optimization passes (optimize is a pass-through)
- 5 tests total, all in `db-sql`
