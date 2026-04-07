# Current Architecture

This repository is a Rust 2024 workspace for an analytical database prototype.
Today, the implemented part of the system is the in-memory catalog and columnar
storage layer. The SQL, optimizer, and execution layers exist, but they are
mostly placeholders and do not currently read stored data.

## Repo Status

- The workspace default member is `crates/db-cli`.
- `cargo check` succeeds for the default member set.
- `cargo run -p db-cli` succeeds and currently prints `QueryResult`.
- `cargo check --workspace --all-targets` fails because
  `crates/db-execution/benches/query.rs` still targets an older execution API.
- There are no `#[test]` tests in the workspace.

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

There are two very different layers in the codebase.

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

### Placeholder query path

```text
SQL text
  -> db_sql::parse()
  -> db_sql::translate()
  -> db_sql::bind()
  -> db_optimizer::build_plan()
  -> db_optimizer::optimize()
  -> db_execution::physical_plan()
  -> db_execution::execute()
  -> db_types::QueryResult
```

Only `db_sql::parse()` does meaningful work today. It uses `sqlparser`,
requires exactly one SQL statement, and returns a wrapper around the parsed AST.
Everything after that currently returns `Default` placeholder values and ignores
most or all of its inputs.

As a result, the CLI exercises the crate boundaries, but it does not execute a
real query over stored table data.

## Crate Responsibilities

### `db-types`

Shared schema, datatype, and placeholder plan/result types.

- `DataTypeKind` supports `U64`, `U32`, `U8`, `I64`, `I32`, `I8`, `F32`,
  `F64`, and `BOOL`.
- `ColumnDefinition` stores column name, type, and a cached width.
- `TableSchema` stores ordered columns and precomputes `row_size_bytes`.
- `UnresolvedPlan` and `ResolvedPlan` are empty structs.
- `LogicalPlan` currently has only `LogicalPlan::None`.
- `PhysicalPlan` currently has only `PhysicalPlan::None`.
- `QueryResult` is currently an empty struct.
- `OutputTable` lives in this crate, not `db-execution`; it wraps a `String`
  and `from_query_result()` currently returns an empty output string.

### `db-storage`

Implemented in-memory columnar storage primitives.

- `TablePartition` owns one `ColumnSegment` per schema column plus `row_count`.
- `TablePartition::insert_rows()` transposes tightly packed row-major input into
  column-major segment buffers.
- `ColumnSegment` stores:
  - dense `Vec<u8>` column data
  - source column index
  - `ZoneMap`
  - optional Bloom filter
  - HyperLogLog cardinality estimator
- `ZoneMap` stores fixed-width min/max bytes for supported primitive types.
- `varlen.rs` contains an unfinished arena-backed string sketch, but it is not
  compiled because `pub mod varlen;` is commented out in `lib.rs`.

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

Parser front-end plus placeholder translation/binding.

- `parse(sql)` uses `sqlparser::parser::Parser::parse_sql`.
- `parse(sql)` panics on parse failure or if the input contains anything other
  than exactly one SQL statement.
- `translate()` ignores the parsed AST and returns `UnresolvedPlan::default()`.
- `bind()` ignores both the unresolved plan and the catalog and returns
  `ResolvedPlan::default()`.

This means the crate currently validates syntax and statement count, but it
does not yet produce a semantic representation of the query.

### `db-optimizer`

Planner/optimizer boundary is present, but behavior is placeholder-only.

- `build_plan()` ignores `ResolvedPlan` and returns `LogicalPlan::default()`.
- `optimize()` ignores the incoming logical plan and returns
  `LogicalPlan::default()`.

### `db-execution`

Physical planning and execution APIs exist, but they do not use storage.

- `physical_plan()` ignores `LogicalPlan` and returns `PhysicalPlan::default()`.
- `execute()` ignores both the physical plan and the `Database`, and returns
  `QueryResult::default()`.

There is no implemented scan, projection, predicate evaluation, aggregation, or
result materialization path in this crate right now.

### `db-cli`

The current demo binary wires the crates together:

1. defines a local `Vec3` row type
2. creates an in-memory `vec3` table
3. inserts four rows
4. flushes the table write buffer
5. runs the placeholder SQL pipeline on `SELECT col1 FROM vec3`
6. prints the `Debug` form of the returned `QueryResult`

Because execution is stubbed, the printed output is currently just:

```text
QueryResult
```

The binary's `main()` is compiled only on 64-bit targets. There is no 32-bit
fallback `main()`.

### `db-server`

Networking scaffold only.

- The crate exposes no public entrypoint.
- It contains a private `run_server_main()` async function.
- That function binds `0.0.0.0:8080`, converts the socket into a Tokio
  `TcpListener`, and accepts one connection.
- There is no protocol handling and no integration with the database crates.

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

The actual crate graph is not a single straight pipeline. The important edges
are:

```text
db-types
  -> db-storage
  -> db-catalog

db-sql -> { db-types, db-catalog }
db-execution -> { db-types, db-storage, db-catalog }
db-optimizer -> { db-types, db-catalog, db-execution }
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
- `crates/db-execution/benches/query.rs` is stale and does not build against
  the current code. It still expects:
  - `db_execution::OutputTable`
  - `PhysicalPlan::TableScan`
  - an `execute(&plan, &db)` signature
- The top-level `build.rs` requires a 64-bit target, but the workspace root is
  not a package, so that `build.rs` is currently inactive.

## Current Gaps

The largest gaps between the current repository and a functioning database
engine are:

- no durable storage
- no WAL integration
- no MVCC implementation
- no scheduler implementation
- no real SQL translation or binding
- no logical or physical planning beyond placeholder enums
- no execution engine over stored partitions
- no typed error model; most failure paths panic
- no query tests
- one stale benchmark that keeps `--workspace --all-targets` from passing
