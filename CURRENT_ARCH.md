# Architecture

A columnar analytical database engine written in Rust (Edition 2024).
In-memory, single-writer, designed for high-throughput scan workloads.

## Crate Layout

```
db/
├── crates/
│   ├── db-types        Type system, schema, column definitions
│   ├── db-storage      TableParititions, column segments, zone maps, bloom filters
│   ├── db-catalog      Database and table management, write buffering
│   ├── db-execution    Physical plan, table scan executor, ASCII output
│   ├── db-optimizer    Logical plan, logical→physical planner (pass-through)
│   ├── db-sql          SQL parser and binder (SELECT * only)
│   ├── db-cli          CLI entry point (default workspace member)
│   ├── db-mvcc         Multi-version concurrency control (empty)
│   ├── db-scheduler    Task scheduling (empty)
│   ├── db-wal          Write-ahead logging (empty)
│   └── db-server       TCP server skeleton (Tokio + socket2)
├── CLAUDE.md           Project invariants and coding standards
└── CURRENT_ARCH.md     This file
```

The full query pipeline is operational end-to-end:
`db-sql → db-optimizer → db-execution → db-catalog → db-storage → db-types`.
The remaining crates (`db-mvcc`, `db-wal`, `db-scheduler`) are empty; `db-server`
has socket initialisation only.

## Dependency Graph

```
db-types (no deps)
 └─ db-storage    ← bytemuck, rapidhash, fastbloom, cardinality-estimator
     └─ db-catalog ← rapidhash
         └─ db-execution
             └─ db-optimizer
                 └─ db-sql ← sqlparser

db-mvcc      ← db-types, db-storage  (empty)
db-wal       ← db-types, db-storage  (empty)
db-scheduler ← db-types, db-execution (empty)
db-server    ← tokio, socket2

db-cli ← db-types, db-storage, db-catalog, db-execution,
         db-optimizer, db-sql, bytemuck
```

## Storage Model

The engine stores data **column-major**. Rows arrive as tightly-packed
byte arrays (array-of-structs), get buffered, then transposed into
per-column byte vectors inside immutable table parititions.

```
Database
 └─ tables: HashMap<String, DBTable>      (rapidhash::fast)
     └─ DBTable
         ├─ write_buffer: Vec<u8>          row-major staging area
         ├─ table_parititions: Vec<TableParitition>  column-major partitions
         ├─ schema: TableSchema
         ├─ meta: TableMeta                name + id
         └─ stats: TableStatistics         (empty placeholder)
```

### Write Path

1. `Database::insert(table, bytes)` delegates to `DBTable::insert`.
2. `DBTable::insert` validates the byte length (must be a multiple of
   `row_size_bytes`) then extends `write_buffer`.
3. When `write_buffer.len() / row_size_bytes >= 4 096`, the table
   auto-flushes via `flush_write_buffer()`.
4. `flush_write_buffer` drains the buffer and calls
   `write_rows_to_table_parititions`, which distributes rows across
   table parititions, allocating new ones when the active partition is
   full.

### TableParitition

Each table paritition holds up to **131 072 rows** (64 × 2 048). It contains
one `ColumnSegment` per schema column and tracks its own `row_count`.

### Column-Major Transposition

`TableParitition::insert_rows` iterates **columns outer, rows inner**.
For each column, it walks every incoming row and copies that column's
bytes into the column segment's buffer. This keeps each column buffer
hot in cache during its entire fill phase.

```
for column in schema.columns():
    col.reserve(row_count * col_byte_width)
    for row in rows:
        col.push_dtype_val(row[col_start..col_end])
```

### ColumnSegment

```rust
pub struct ColumnSegment {
    data: Vec<u8>,                                          // dense column bytes
    column_def_index: usize,                                // position in schema
    zone_map: ZoneMap,                                      // per-segment min/max
    bloom: Option<BloomFilter<rapidhash::quality::RandomState>>, // membership filter
    hll: CardinalityEstimator<[u8], rapidhash::quality::RapidHasher<'static>>, // HLL++
}
```

Every value pushed feeds all three auxiliary structures:

- **ZoneMap** — type-aware min/max comparison via `bytemuck` zero-copy
  cast; supports range-based segment pruning.
- **BloomFilter** — 64-bit, 1-hash membership filter; supports
  point-lookup segment pruning.
- **HyperLogLog++** — O(1) approximate distinct-count per column
  segment without a separate pass.

## Type System

All types are fixed-width. The `DataTypeKind` enum covers:

| Kind | Width |
|------|-------|
| U64, I64, F64 | 8 bytes |
| U32, I32, F32 | 4 bytes |
| U8, I8, BOOL | 1 byte |

Byte order is little-endian throughout. `DataTypeKind::format_bytes`
deserialises a column slice to a display string; floats use 6 decimal
places. Type names are parsed case-insensitively.

### String Support (Designed, Not Integrated)

`varlen.rs` in `db-storage` contains an unconnected design for
variable-length strings:

- **StringRef**: 16-byte aligned. 4-byte length + 12-byte payload.
  Strings ≤ 12 bytes are inlined; longer strings store a 4-byte
  prefix plus an arena index and offset.
- **StringBuffer**: per-row-group arena backed by a linked list of
  1 MB `ArenaBuffer` blocks.

All methods are `todo!()`.

## Schema

```
TableSchema
 ├─ columns: Vec<ColumnDefinition>
 └─ row_size_bytes: usize        sum of column widths

ColumnDefinition
 ├─ dtype: DataTypeKind
 ├─ width: u32
 └─ name: String
```

`row_size_bytes` is computed once at schema creation. It is used
everywhere to validate insert lengths and to stride over row-major
input buffers.

## Query Pipeline

The full pipeline is working for `SELECT *` queries:

```
SQL string
    │  db-sql::bind()
    ▼
LogicalPlan::Scan { table_name, column_indices }
    │  db-optimizer::plan()
    ▼
PhysicalPlan::TableScan { table_name, column_indices }
    │  db-execution::execute()
    ▼
QueryResult { columns: Vec<ResultColumn>, row_count }
    │  OutputTable::from_query_result()
    ▼
ASCII table string
```

### SQL Layer (`db-sql`)

`bind(sql, database)` uses `sqlparser` (GenericDialect) to parse the
SQL string. It validates:

- Exactly one statement.
- A `SELECT` body (not UNION / INTERSECT / EXCEPT).
- Exactly one table in `FROM`, no JOINs.
- `SELECT *` only — named columns are rejected.

On success it returns `LogicalPlan::Scan` with the resolved table name
and column indices `0..column_count`.

Currently uses `panic!` for all error paths.

### Optimizer (`db-optimizer`)

`plan(logical)` performs a **direct pass-through** from
`LogicalPlan::Scan` to `PhysicalPlan::TableScan`. No rewrites, no
cost model. The separation exists for future extension.

### Execution (`db-execution`)

`execute(plan, database)` dispatches to `execute_table_scan`:

1. Flushes the table's write buffer.
2. Creates a `ResultColumn` (name + type + empty `Vec<u8>`) for each
   selected column.
3. Iterates all table parititions; for each partition appends
   `segment.data()[..byte_count]` into the matching `ResultColumn`.
4. Returns `QueryResult { columns, row_count }`.

`OutputTable::from_query_result` formats the result as an ASCII table:
auto-sized column widths, separator lines, and aligned cell values
rendered via `DataTypeKind::format_bytes`.

## Concurrency Model

The engine is **single-writer, single-threaded**. `Database` takes
`&mut self` for all mutations. No `Arc`, `Mutex`, or interior
mutability is used.

A comment in `table.rs` marks the planned evolution:

```rust
// will become: table_parititions: ArcSwap<Vec<Arc<TableParitition>>>,
```

## Platform Requirements

`main.rs` is gated on `#[cfg(target_pointer_width = "64")]`. On
32-bit targets the binary prints an error and exits.

## Memory Bounds

| Component | Capacity | Enforcement |
|-----------|----------|-------------|
| Write buffer | 4 096 rows | Auto-flush when full |
| TableParitition | 131 072 rows | Allocate new when full |
| Table count | u32::MAX | Panic on overflow |
| Row width | Computed at schema creation | Validated on each insert |

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| bytemuck 1.25 | Zero-copy cast between `#[repr(C)]` structs and `&[u8]` |
| rapidhash 4.4 | Fast hashing for `HashMap`, ZoneMap, and BloomFilter |
| cardinality-estimator 1.0 | HyperLogLog++ distinct-count per column |
| fastbloom 0.17 | Bloom filter for membership-based segment pruning |
| sqlparser 0.61 | SQL parsing (AST + binder) |
| tokio 1.50 | Async runtime for future server |
| socket2 0.6 | Low-level socket configuration |
| criterion 0.5 | Benchmarking (dev dependency) |

## Benchmarking

`crates/db-catalog/benches/insert.rs` measures insert throughput with
Criterion:

- **vec3_one_at_a_time**: 100 000 rows inserted individually.
- **vec3_bulk**: 100 000 rows inserted in a single `&[u8]` slice.

Both use a `Vec3 { x: f32, y: f32, z: f32 }` schema. Sample size is
20 iterations. Throughput is reported in elements/sec.

## Data Flow Diagram

```
                       raw bytes (&[u8])
                            │
                            ▼
                   ┌─────────────────┐
                   │  write_buffer   │  row-major, up to 4096 rows
                   └────────┬────────┘
                            │ flush (auto at capacity or explicit)
                            ▼
                ┌───────────────────────┐
                │  write_rows_to_table_ │  split across table
                │  parititions()        │  parititions, allocate new
                │                       │  if full
                └───────────┬───────────┘
                            │
              ┌─────────────┼─────────────┐
              ▼             ▼             ▼
        ┌──────────┐  ┌──────────┐  ┌──────────┐
        │ ColSeg 0 │  │ ColSeg 1 │  │ ColSeg N │   column-major
        │ data[]   │  │ data[]   │  │ data[]   │   dense bytes
        │ zone_map │  │ zone_map │  │ zone_map │   min/max per segment
        │ bloom    │  │ bloom    │  │ bloom    │   membership filter
        │ hll      │  │ hll      │  │ hll      │   HLL++ estimator
        └──────────┘  └──────────┘  └──────────┘
```

## File Index

```
crates/db-types/src/
  lib.rs                 Module declarations
  data_type.rs           DataTypeKind: type enum, parse, byte_width, format_bytes
  column_definition.rs   ColumnDefinition: name, type, width
  table_schema.rs        TableSchema: column definitions, row width

crates/db-storage/src/
  lib.rs                 Module declarations
  table_paritition.rs    TableParitition: column-major transposition, capacity tracking
  segment.rs             ColumnSegment: dense bytes + ZoneMap + BloomFilter + HLL
  zone_map.rs            ZoneMap: type-aware min/max tracking per segment
  varlen.rs              String arena design (not connected)

crates/db-catalog/src/
  lib.rs                 Module declarations
  database.rs            Database: table map, insert, access
  table.rs               DBTable: write buffer, flush, table paritition management
  statistics.rs          TableStatistics (empty placeholder)

crates/db-catalog/benches/
  insert.rs              Criterion insert benchmarks

crates/db-execution/src/
  lib.rs                 Module declarations
  physical_plan.rs       PhysicalPlan: TableScan variant
  query_result.rs        QueryResult, ResultColumn
  execute.rs             execute(): dispatch + execute_table_scan()
  output.rs              OutputTable: ASCII table renderer

crates/db-optimizer/src/
  lib.rs                 Module declarations
  logical_plan.rs        LogicalPlan: Scan variant
  planner.rs             plan(): LogicalPlan → PhysicalPlan (pass-through)

crates/db-sql/src/
  lib.rs                 bind(): SQL string → LogicalPlan (SELECT * only)

crates/db-cli/src/
  main.rs                CLI entry point, end-to-end demo (64-bit only)

crates/db-server/src/
  lib.rs                 Tokio TCP accept skeleton (socket2 init only)

crates/db-mvcc/src/
  lib.rs                 (empty — scaffolding)

crates/db-scheduler/src/
  lib.rs                 (empty — scaffolding)

crates/db-wal/src/
  lib.rs                 (empty — scaffolding)
```
