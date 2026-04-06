# Architecture

This repository is a small columnar analytical database engine in Rust 2024.
The implemented path is an in-memory, single-threaded scan engine with a working
`SELECT * FROM table` pipeline and a demo CLI.

## Workspace

```
db/
├── crates/
│   ├── db-types
│   ├── db-storage
│   ├── db-catalog
│   ├── db-execution
│   ├── db-optimizer
│   ├── db-sql
│   ├── db-cli
│   ├── db-server
│   ├── db-mvcc
│   ├── db-scheduler
│   └── db-wal
├── Cargo.toml
├── Justfile
├── CLAUDE.md
└── CURRENT_ARCH.md
```

`db-cli` is the default workspace member and currently serves as the end-to-end
demo binary.

## Implemented Pipeline

The working query path is:

```
SQL text
  -> db-sql::bind()
  -> db-optimizer::plan()
  -> db-execution::execute()
  -> db-execution::output::OutputTable
  -> ASCII table output
```

The working write path is:

```
row-major bytes
  -> db_catalog::Database::insert()
  -> DBTable write buffer
  -> DBTable::flush_write_buffer()
  -> db_storage::TableParitition
  -> per-column ColumnSegment storage
```

Important limitation: reads do not flush pending writes. Callers must flush a
table explicitly before planning or execution if they want buffered rows to be
visible. The CLI does this manually.

## Crate Responsibilities

### `db-types`

Owns the fixed-width type system and schema metadata.

- `DataTypeKind` supports `U64`, `U32`, `U8`, `I64`, `I32`, `I8`, `F32`,
  `F64`, and `BOOL`
- `ColumnDefinition` stores column name, type, and cached width
- `TableSchema` stores ordered columns and precomputed `row_size_bytes`

Type parsing is case-insensitive. Display formatting is little-endian and
formats floats with 6 decimal places.

### `db-storage`

Owns the physical in-memory column storage.

- `TableParitition` is the row-group container
- `ColumnSegment` stores one dense byte vector per column
- `ZoneMap` tracks per-segment min/max bytes
- `varlen.rs` exists but is not compiled or integrated

The crate intentionally exposes the misspelled `TableParitition` type because
that is the current code and public API.

### `db-catalog`

Owns table lifecycle and buffered inserts.

- `Database` maps table names to `DBTable`
- `DBTable` owns schema, partitions, a write buffer, and placeholder stats
- `TableStatistics` is currently an empty struct

Table names are unique. Table ids are assigned from the current table count and
panic if the count would exceed `u32::MAX`.

### `db-execution`

Owns physical plan execution and text output formatting.

- `PhysicalPlan` currently has only `TableScan`
- `execute()` walks table partitions and builds a `QueryResult`
- `OutputTable` renders the result as an ASCII grid

`QueryResult` is chunked and zero-copy over stored partition data. It does not
materialize a fresh contiguous buffer per result column.

### `db-optimizer`

Owns the logical-to-physical planning boundary.

- `LogicalPlan` currently has only `Scan`
- `plan()` is a direct passthrough to `PhysicalPlan::TableScan`

There are no rewrites, no cost model, and no rule system yet.

### `db-sql`

Owns SQL parsing and binding with `sqlparser`.

Current binder behavior:

- exactly one statement
- statement must be a query
- query body must be a simple `SELECT`
- exactly one table in `FROM`
- no joins
- projection must be `SELECT *`

Binding resolves the table name against the catalog and returns all column
indices for the table schema.

### `db-cli`

Contains the current executable demo.

It:

1. defines a local `Vec3` row type
2. creates an in-memory `vec3` table
3. inserts four rows
4. flushes the write buffer
5. runs `SELECT * FROM vec3`
6. prints the formatted output

The binary only runs on 64-bit targets. On 32-bit targets it prints a message
and exits.

### `db-server`

This crate is only a stub.

- contains a private `run_server_main()` async function
- creates and binds a TCP socket on `0.0.0.0:8080`
- converts it into a Tokio `TcpListener`
- awaits a single accepted connection

There is no public entrypoint, no protocol, and no integration with the
database crates.

### `db-mvcc`, `db-scheduler`, `db-wal`

These crates have manifests but empty `lib.rs` files. They are scaffolding only.

## Data Model

The engine ingests tightly packed row-major bytes and stores them in
column-major partitions.

```
Database
└── HashMap<String, DBTable, rapidhash::fast::RandomState>
    └── DBTable
        ├── meta: TableMeta { name, id }
        ├── schema: TableSchema
        ├── table_parititions: Vec<TableParitition>
        ├── stats: TableStatistics
        └── write_buffer: Vec<u8>
```

### Write Buffer

- insert input must be a multiple of `schema.row_size_bytes()`
- rows accumulate in `write_buffer`
- the buffer auto-flushes once it reaches 4,096 rows worth of bytes
- `flush_write_buffer()` swaps the buffer out, writes it into partitions, then
  restores the allocation for reuse

### Partitions

Each `TableParitition` holds:

- `columns: Vec<ColumnSegment>`
- `row_count: usize`

Capacity is fixed at `64 * 2048 = 131_072` rows per partition.

When a flush would overflow the current partition, `DBTable` allocates a new
partition and continues writing the remaining rows.

### Column Segments

Each `ColumnSegment` stores:

- `data: Vec<u8>`
- `column_def_index: usize`
- `zone_map: ZoneMap`
- `bloom: Option<BloomFilter<rapidhash::quality::RandomState>>`
- `hll: CardinalityEstimator<[u8], rapidhash::quality::RapidHasher<'static>>`

Every inserted value:

- appends bytes to the dense column buffer
- updates the Bloom filter when present
- updates the HyperLogLog estimator
- updates the zone map using the column data type

### Column-Major Transposition

`TableParitition::insert_rows()` transposes row-major input into column-major
storage by iterating columns outermost and rows innermost.

For each column:

1. compute that column's byte range inside each row
2. reserve space for `row_count * column_width`
3. walk the incoming rows with `chunks_exact(row_byte_width)`
4. append the per-row slice for that column into the target segment

## Query Execution

`db_execution::execute()` currently supports `PhysicalPlan::TableScan` only.

Execution does the following:

1. fetch the table from the catalog
2. build one `ResultColumn` per requested column
3. iterate all stored partitions
4. skip empty partitions
5. push a borrowed chunk for each selected column segment
6. accumulate the total row count

`ResultColumn::row_bytes()` resolves a row by walking chunk boundaries and then
slicing the borrowed segment data.

`OutputTable::from_query_result()` materializes display strings row by row and
computes max widths from both headers and formatted cells before rendering the
ASCII table.

## Storage and Query Semantics

These properties follow from the current code:

- data is in-memory only
- all supported column types are fixed-width
- inserts and most bind errors panic instead of returning typed errors
- queries scan every stored partition; zone maps and Bloom filters are populated
  but not consulted by execution
- only flushed rows are visible to reads
- only full-table scans are supported through `SELECT *`
- there is no delete, update, filter, aggregation, join, or persistence path

## Concurrency Model

The implemented engine is single-threaded and mutation is driven by `&mut self`.
There is no `Arc`, `Mutex`, lock-free structure, or background worker in the
core data path.

Returned query results borrow from the database, so execution is read-only for
the lifetime of the result.

## Dependency Shape

The workspace dependency chain is:

```
db-types
  -> db-storage
  -> db-catalog
  -> db-execution
  -> db-optimizer
  -> db-sql
  -> db-cli
```

Additional crate dependencies:

- `db-server` depends only on `tokio` and `socket2`
- `db-mvcc` depends on `db-types` and `db-storage`
- `db-scheduler` depends on `db-types` and `db-execution`
- `db-wal` depends on `db-types` and `db-storage`

Key third-party libraries in active use:

- `sqlparser` for SQL parsing
- `bytemuck` for typed byte reads in zone maps and demo row casting
- `rapidhash` for the catalog hash map and storage metadata structures
- `fastbloom` for per-segment Bloom filters
- `cardinality-estimator` for per-segment distinct-count estimation
- `criterion` for insert and query benchmarks

## Benchmarks

The repository includes two Criterion benchmark targets:

- `crates/db-catalog/benches/insert.rs`
  - `vec3_one_at_a_time`
  - `vec3_bulk`
- `crates/db-execution/benches/query.rs`
  - `execute`
  - `execute_with_output`

Both benchmark paths operate on a simple three-column `f32` table.

## Current Gaps

The largest gaps between the current codebase and a full database engine are:

- no durable storage or WAL integration
- no MVCC or scheduler implementation
- no typed error model
- no server entrypoint or wire protocol
- no predicate pushdown or index usage despite stored metadata
- no variable-length type support in the compiled engine
