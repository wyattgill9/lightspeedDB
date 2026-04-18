# LightSpeedDB - Current Architecture

Rust 2024 workspace for an analytical database prototype. The implemented
layers are: in-memory columnar storage with write-time statistics, catalog,
and a SQL front-end (parse, translate). Binding, logical planning, physical
planning, and execution are all stubs.

## Repo Status

- Workspace default member is `crates/lsdb-cli`.
- `cargo clippy --workspace --all-targets` succeeds with one pre-existing
  warning in `lsdb-server` (unused function).
- `cargo nextest run --workspace` passes with 2 tests (both in `lsdb-sql`
  `translate` module).
- `cargo run -p lsdb-cli` creates a table, runs SQL through parse and
  translate, and prints the `UnresolvedPlan`. Binding and planning are
  commented out.
- No external error handling crate. All error paths use `panic!`.

## Workspace

```text
lightspeedDB/
├── crates/
│   ├── lsdb-catalog
│   ├── lsdb-cli
│   ├── lsdb-execution
│   ├── lsdb-mvcc
│   ├── lsdb-optimizer
│   ├── lsdb-scheduler
│   ├── lsdb-server
│   ├── lsdb-sql
│   ├── lsdb-storage
│   ├── lsdb-types
│   └── lsdb-wal
├── Cargo.toml          (resolver = "3")
├── justfile
├── flake.nix / flake.lock
├── BENCH_SPECS.md
├── CLAUDE.md
└── CURRENT_ARCH.md
```

## What Works Today

### Implemented write path

```text
typed rows
  -> zerocopy byte cast
  -> Database::insert()
  -> DBTable write_buffer (auto-flushes at 4096 rows)
  -> DBTable::flush_write_buffer()
  -> TablePartition::insert_rows() (columns-outer, rows-inner transposition)
  -> ColumnSegment::push_dtype_val()
      -> ZoneMap::update()
      -> BloomFilter::insert()
      -> CardinalityEstimator::insert()
      -> data.extend_from_slice()
```

This path is real and writes row-major input into in-memory columnar storage
with zone maps, Bloom filters, and HyperLogLog cardinality estimators updated
on every value.

### Implemented query pipeline

```text
SQL text
  -> lsdb_sql::parse()      -- sqlparser wrapper, validates single statement
  -> lsdb_sql::translate()  -- AST -> UnresolvedPlan (SELECT + GROUP BY only)
```

The pipeline stops at `UnresolvedPlan`. Binding, planning, and execution are
all commented out in the CLI.

### Stub layers

```text
UnresolvedPlan
  -> lsdb_sql::bind()               -- ignores input, returns ResolvedPlan::None
  -> lsdb_optimizer::build_plan()    -- ignores input, returns LogicalPlan::None
  -> lsdb_optimizer::optimize()      -- pass-through
  -> lsdb_execution::physical_plan() -- returns PhysicalPlan::None
  -> lsdb_execution::execute()       -- returns QueryResult::default()
```

## Crate Responsibilities

### `lsdb-types`

Shared schema, datatype, plan, execution vector, and result types.

- `DataTypeKind` enum: `U64`, `U32`, `U8`, `I64`, `I32`, `I8`, `F32`, `F64`,
  `BOOL`. Methods: `parse()` (panics on unknown), `byte_width()`,
  `format_bytes()`.
- `ColumnDefinition`: column name, type, and cached width.
- `TableSchema`: ordered columns with precomputed `row_size_bytes`.

Plan types form a staged pipeline with four distinct representations:

- `UnresolvedPlan` -- enum with `Select` variant holding
  `Vec<UnresolvedSelectItem>` projection, table name (`String`), and
  `group_by` (`Vec<UnresolvedExpr>`). `UnresolvedExpr` covers
  `Column(String)` and `Function { name, args }`.
  `UnresolvedFunctionArgs` distinguishes `Star` from
  `Exprs(Vec<UnresolvedExpr>)`.
- `ResolvedPlan` -- stub enum with only a `None` variant.
- `LogicalPlan` -- stub enum with only a `None` variant.
- `PhysicalPlan` -- stub enum with only a `None` variant.

Execution vector types (defined, unused by any real execution):

- `ExecutionVector` trait: `len()`, `as_any()`.
- `FlatVector<T>`: typed `Vec<T>`, implements `ExecutionVector`.
- `DataChunk`: `Vec<Box<dyn ExecutionVector>>`, typed column accessor via
  `column::<T>(idx)`.

- `QueryResult` is an empty struct (placeholder).
- `OutputTable` wraps a `String`; `from_query_result()` returns an empty
  string.

### `lsdb-storage`

In-memory columnar storage primitives.

- `TablePartition` owns one `ColumnSegment` per schema column plus
  `row_count`. Capacity: `64 * 2048 = 131_072` rows.
- `TablePartition::insert_rows()` transposes tightly packed row-major input
  into column-major segment buffers. Iterates columns-outer, rows-inner to
  keep write target hot in L1 cache.
- `ColumnSegment` stores dense `Vec<u8>` column data, source column index,
  and `ColumnSegmentStatistics`.
- `ColumnSegmentStatistics` bundles:
  - `ZoneMap` -- min/max as `[u8; 8]`, uses `zerocopy` for typed comparisons.
  - `BloomFilter` (fastbloom) -- 64 bits, 1 hash function, rapidhash quality.
  - `CardinalityEstimator` -- HyperLogLog with rapidhash.
- `varlen.rs` contains an unfinished DuckDB-style string sketch
  (`StringRef` / `StringBuffer` / `ArenaBuffer`). All functions are
  `todo!()` and the module is commented out of `lib.rs`.

### `lsdb-catalog`

In-memory catalog and table lifecycle layer.

- `Database` stores tables in
  `HashMap<String, DBTable, rapidhash::fast::RandomState>`.
- `create_table_with_schema()` assigns table ids from current table count.
- `insert()` appends bytes into a table-local write buffer.
- `flush_table_writes()` drains the write buffer into partitions.
- `table()` / `table_mut()` panic if the table does not exist.
- `get_table()` is the only non-panicking lookup path.

`DBTable` owns:

- `meta: TableMeta { name, id }`
- `schema: TableSchema`
- `table_partitions: Vec<TablePartition>`
- `stats: TableStatistics` (empty struct, no fields)
- `write_buffer: Vec<u8>` (auto-flushes at `4096 * row_size_bytes`)

Write buffer uses `mem::swap` to drain without re-allocating, then restores
the cleared allocation for reuse.

### `lsdb-sql`

SQL front-end with real parsing and translation; binding is a stub.

- `parse(sql)` uses `sqlparser` with `GenericDialect`. Panics on parse
  failure or if the input contains anything other than exactly one statement.
- `translate(ParsedStatement)` converts the parsed AST into an
  `UnresolvedPlan`. Supports `SELECT` with projections and `GROUP BY`.
  Explicitly rejects: `WHERE`, `ORDER BY`, `LIMIT`, `DISTINCT`, `JOIN`,
  window functions, CTEs, subqueries, and many other constructs.
- `bind(UnresolvedPlan, &Database)` is a stub: ignores both arguments,
  returns `ResolvedPlan::None`.

2 tests cover the parse-translate pipeline for GROUP BY queries with
`count(*)` and `avg()`.

### `lsdb-optimizer`

Stub logical plan construction and optimization.

- `build_plan(ResolvedPlan) -> LogicalPlan` ignores its input, returns
  `LogicalPlan::None`.
- `optimize(LogicalPlan) -> LogicalPlan` is a pass-through.

### `lsdb-execution`

Stub physical planning and execution.

- `physical_plan()` ignores `LogicalPlan`, returns `PhysicalPlan::None`.
- `execute()` ignores the physical plan and the `Database`, returns
  `QueryResult::default()`.

The execution benchmark loads 100K `Vec3` rows and benchmarks `execute()`,
but exercises no real execution logic since execution is stubbed.

### `lsdb-cli`

Demo binary (64-bit only) that wires crates together:

1. Creates a `trips` table with columns `(cab_type: u8, passenger_count: u8,
   total_amount: f64)`.
2. Runs `SELECT passenger_count, avg(total_amount) FROM trips GROUP BY
   passenger_count` through parse and translate.
3. Prints the `Debug` form of the `UnresolvedPlan`.

The `bind` / `build_plan` / `optimize` calls are commented out.

### `lsdb-server`

Networking scaffold only. Contains a private unused `run_server_main()` async
function that binds `0.0.0.0:8080` via `socket2`, converts to a Tokio
`TcpListener`, and accepts one connection. No protocol handling, no
integration with database crates.

### `lsdb-mvcc`, `lsdb-scheduler`, `lsdb-wal`

These crates have manifests and dependency edges, but their `lib.rs` files
are empty.

## Storage Model

```text
Database
└── HashMap<String, DBTable, rapidhash::fast::RandomState>
    └── DBTable
        ├── meta: TableMeta { name, id }
        ├── schema: TableSchema
        ├── write_buffer: Vec<u8>          (row-major staging)
        ├── stats: TableStatistics         (empty)
        └── table_partitions: Vec<TablePartition>
            └── TablePartition { row_count }
                └── columns: Vec<ColumnSegment>
                    ├── data: Vec<u8>               (dense column bytes)
                    ├── column_def_index: usize
                    └── stats: ColumnSegmentStatistics
                        ├── zone_map: ZoneMap        (min/max [u8; 8])
                        ├── bloom: BloomFilter        (fastbloom, 64-bit)
                        └── hll: CardinalityEstimator (HyperLogLog)
```

### Constants

| Constant | Value | Purpose |
|---|---|---|
| `TABLE_PARTITION_CAPACITY` | 131,072 rows | Max rows per partition |
| `CAPACITY_ROWS_WRITE_BUFFER` | 4,096 rows | Auto-flush threshold |

### Ingestion

- Input must be a multiple of `schema.row_size_bytes()`.
- Rows accumulate in `write_buffer`.
- Auto-flush at 4096 rows drains the buffer into partitions.
- Transposition is columns-outer / rows-inner for cache locality.
- Statistics (zone map, Bloom, HLL) update on every inserted value.
- Statistics are built on write but never consulted by any read path.

## Dependency Shape

```text
lsdb-types  (no deps)
  -> lsdb-storage  (+ zerocopy, cardinality-estimator, fastbloom, rapidhash)
  -> lsdb-catalog  (+ rapidhash)

lsdb-sql        -> { lsdb-types, lsdb-catalog, sqlparser }
lsdb-optimizer  -> { lsdb-types }
lsdb-execution  -> { lsdb-types, lsdb-storage, lsdb-catalog }
lsdb-cli        -> all of the above

lsdb-mvcc       -> { lsdb-types, lsdb-storage }      (empty)
lsdb-scheduler  -> { lsdb-types, lsdb-execution }    (empty)
lsdb-wal        -> { lsdb-types, lsdb-storage }      (empty)
lsdb-server     -> { tokio, socket2 }                 (no DB deps)
```

## Key External Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `zerocopy` | 0.8 | Zero-copy byte casting (replaced bytemuck) |
| `rapidhash` | 4.4 | Fast hashing for HashMap, Bloom, HLL |
| `fastbloom` | 0.17 | Bloom filter |
| `cardinality-estimator` | 1 | HyperLogLog |
| `sqlparser` | 0.61 | SQL AST parsing |
| `tokio` | 1.50 | Async runtime (server only) |
| `socket2` | 0.6 | Socket configuration (server only) |
| `criterion` | 0.5 | Benchmarks |

## Build and Benchmark Notes

- `justfile` defines `build`, `run`, `bench`, `check`, `test`, and `fmt`.
- `crates/lsdb-catalog/benches/insert.rs` exercises the real write path with
  100K `Vec3` rows (single-row and bulk variants).
- `crates/lsdb-execution/benches/query.rs` compiles and runs but exercises
  no real execution logic.
- Nix devShell (`flake.nix`) provides Rust nightly and cargo-nextest.

## Current Gaps

The largest gaps between the current repository and a functioning database:

- No typed error model; all failure paths panic (CLAUDE.md mandates `snafu`)
- SQL binding is a stub (no name resolution, no type checking)
- No logical planning (`LogicalPlan` is stub enum with only `None`)
- No physical planning (`PhysicalPlan` is stub enum with only `None`)
- No execution engine over stored partitions
- SQL support limited to SELECT with GROUP BY (no WHERE, ORDER BY, LIMIT,
  JOIN, DISTINCT)
- No durable storage, no WAL integration
- No MVCC implementation
- No scheduler implementation
- No network protocol (TCP scaffold only)
- Storage statistics (zone maps, Bloom, HLL) built on write but never read
- Variable-length string support sketched but unimplemented
- `QueryResult` and `OutputTable` are empty placeholders
- 2 tests total, all in `lsdb-sql` translate module
