# Architecture

A columnar analytical database engine written in Rust (Edition 2024).
In-memory, single-writer, designed for high-throughput scan workloads.

## Crate Layout

```
db/
├── crates/
│   ├── db-types        Type system, schema, column definitions
│   ├── db-storage      Row groups, column segments, zone maps, bloom filters
│   ├── db-catalog      Database and table management, write buffering
│   ├── db-execution    Query execution and ASCII output formatting
│   ├── db-cli          CLI entry point (default workspace member)
│   ├── db-buffer       Memory buffering layer (scaffolding)
│   ├── db-mvcc         Multi-version concurrency control (scaffolding)
│   ├── db-optimizer    Query optimizer (scaffolding)
│   ├── db-scheduler    Task scheduling (scaffolding)
│   ├── db-wal          Write-ahead logging (scaffolding)
│   ├── db-server       TCP server skeleton (Tokio + socket2)
│   └── db-sql          SQL parser wrapper (sqlparser)
├── CLAUDE.md           Project invariants and coding standards
└── CURRENT_ARCH.md     This file
```

The core pipeline is `db-types → db-storage → db-catalog → db-execution
→ db-cli`. The remaining crates are scaffolding with dependency wiring
in place but no logic yet.

## Dependency Graph

```
db-cli ─────────────────────────────────────────┐
├── db-types                                    │
├── db-storage                                  │
│   └── db-types                                │
├── db-catalog                                  │
│   ├── db-types                                │
│   └── db-storage                              │
└── db-execution                                │
    ├── db-types                                │
    ├── db-storage                              │
    └── db-catalog                              │
                                                │
db-server ── (tokio, socket2)                   │
db-sql    ── (sqlparser)                        │
db-buffer ── db-types, db-storage               │
db-mvcc   ── db-types, db-storage               │
db-optimizer ── db-types, db-catalog            │
db-scheduler ── db-types, db-execution          │
db-wal    ── db-types, db-storage               │
```

## Storage Model

The engine stores data **column-major**. Rows arrive as tightly-packed
byte arrays (array-of-structs), get buffered, then transposed into
per-column byte vectors inside immutable row groups.

```
Database
 └─ tables: HashMap<String, DBTable>      (rapidhash::fast)
     └─ DBTable
         ├─ write_buffer: Vec<u8>          row-major staging area
         ├─ row_groups: Vec<RowGroup>      column-major partitions
         ├─ schema: TableSchema
         ├─ meta: TableMeta                name + id
         └─ stats: TableStatistics         (placeholder)
```

### Write Path

1. `Database::insert(table, bytes)` appends raw bytes to the table's
   write buffer.
2. When the buffer reaches **4 096 rows**, it auto-flushes.
3. `flush_write_buffer()` swaps the buffer out (to avoid aliasing),
   then calls `write_rows_to_segments`.
4. Rows are split across row groups. If the active row group is full,
   a new one is allocated.

### RowGroup

Each row group holds up to **131 072 rows** (64 × 2 048). It contains
one `ColumnSegment` per schema column. The row group tracks its own
`row_count` and derives `rows_available` from the fixed capacity.

### Column-Major Transposition

`RowGroup::insert_rows` iterates **columns outer, rows inner**.
For each column, it walks every row and copies that column's bytes
into the column segment. This keeps each column buffer hot in L1/L2
cache during its entire fill rather than alternating between N buffers
per row.

```
for column in schema.columns():
    col.reserve(row_count * col_byte_width)
    for row in rows:
        col.push_dtype_val(row[col_start..col_end])
```

### ColumnSegment

```rust
pub struct ColumnSegment {
    data: Vec<u8>,                   // dense column bytes
    column_def_index: usize,         // position in schema
    zone_map: ZoneMap,               // per-segment min/max index
    bloom: Option<BloomFilter<...>>, // membership filter
    hll: CardinalityEstimator<...>,  // HyperLogLog++ per column
}
```

Every value pushed into a column feeds all three structures:

- **ZoneMap** — type-aware min/max comparison via `bytemuck` zero-copy
  cast; supports range-based segment pruning.
- **BloomFilter** — membership filter backed by `fastbloom`; supports
  point-lookup segment pruning (skip segments where a value is
  provably absent).
- **HyperLogLog++** — O(1) approximate distinct-count per column per
  segment without a separate pass.

Both ZoneMap and BloomFilter use `rapidhash::quality` for hashing.

## Type System

All types are fixed-width. The `DataTypeKind` enum covers:

| Kind | Width |
|------|-------|
| U64, I64, F64 | 8 bytes |
| U32, I32, F32 | 4 bytes |
| U8, I8, BOOL | 1 byte |

Byte order is little-endian throughout. Type names are parsed
case-insensitively. `format_bytes` deserialises a column slice back
into a display string for output.

### String Support (Designed, Not Integrated)

`varlen.rs` in `db-storage` contains an unconnected design for
variable-length strings:

- **StringRef**: 16-byte aligned. 4-byte length + 12-byte payload.
  Strings ≤ 12 bytes are inlined; longer strings store a 4-byte
  prefix plus an arena index and offset.
- **StringBuffer**: Per-row-group arena backed by a linked list of
  1 MB `ArenaBuffer` blocks.

All methods are `todo!()`.

## Schema

```
TableSchema
 └─ columns: Vec<ColumnDefinition>
 └─ row_size_bytes: usize        sum of column widths

ColumnDefinition
 └─ dtype: DataTypeKind
 └─ width: u32
 └─ name: String
```

`row_size_bytes` is computed once at schema creation. It is used
everywhere to validate insert lengths and to stride over row-major
input buffers.

## Metadata and Statistics

- **TableMeta**: name (`String`) and monotonic id (`u32`, assigned from
  `tables.len()`).
- **TableStatistics**: empty struct, reserved for future counters
  (row count, byte count, segment count).

## Query Execution

Currently limited to full-table scans:

- **OutputTable** in `db-execution` flushes the write buffer, iterates
  all row groups, reconstructs rows from columnar data, and renders
  an ASCII table with auto-sized columns.
- No predicate evaluation, aggregation, or segment pruning exists yet.

## SQL Layer

`db-sql` wraps `sqlparser` (v0.61) to parse SQL strings into an AST
and returns the debug representation. No evaluation.

## Network Layer

`db-server` binds a TCP socket on `0.0.0.0:8080` using `socket2` for
low-level control and `tokio::net::TcpListener` for async accept. It
currently accepts one connection and exits. No wire protocol, message
framing, or query routing is implemented.

## Concurrency Model

The engine is **single-writer, single-threaded** today. `Database`
takes `&mut self` for all mutations. No `Arc`, `Mutex`, or interior
mutability is used.

A comment in `table.rs` marks the planned evolution:

```rust
// will become: row_groups: ArcSwap<Vec<Arc<RowGroup>>>,
```

This points toward a future model where readers snapshot row groups
atomically while a single writer appends new segments.

## Platform Requirements

`main.rs` is gated on `#[cfg(target_pointer_width = "64")]`. On
32-bit targets the binary prints an error and exits.

## Memory Bounds

| Component | Capacity | Enforcement |
|-----------|----------|-------------|
| Write buffer | 4 096 rows | Auto-flush when full |
| Row group | 131 072 rows | Allocate new when full |
| Table count | u32::MAX | Panic on overflow |
| Row width | Computed at schema creation | Validated on each insert |

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| bytemuck 1.25 | Zero-copy cast between `#[repr(C)]` structs and `&[u8]` |
| rapidhash 4.4 | Fast hashing for `HashMap`, ZoneMap, and BloomFilter |
| cardinality-estimator 1.0 | HyperLogLog++ distinct-count per column |
| fastbloom 0.17 | Bloom filter for membership-based segment pruning |
| sqlparser 0.61 | SQL parsing (AST only) |
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
                │  write_rows_to_       │  split across row groups,
                │  segments()           │  allocate new if full
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
  data_type.rs           DataTypeKind: type enum, parse, format
  column_definition.rs   ColumnDefinition: name, type, width
  table_schema.rs        TableSchema: column definitions, row width

crates/db-storage/src/
  lib.rs                 Module declarations
  row_group.rs           RowGroup: column-major transposition
  segment.rs             ColumnSegment: dense bytes + ZoneMap + BloomFilter + HLL
  zone_map.rs            ZoneMap: type-aware min/max tracking per segment
  varlen.rs              String arena design (not connected)

crates/db-catalog/src/
  lib.rs                 Module declarations
  database.rs            Database: table map, insert, access
  table.rs               DBTable: write buffer, flush, row group management
  statistics.rs          TableStatistics (placeholder)

crates/db-catalog/benches/
  insert.rs              Criterion insert benchmarks

crates/db-execution/src/
  lib.rs                 Module declarations
  output.rs              OutputTable: ASCII table renderer

crates/db-cli/src/
  main.rs                CLI entry point, example usage (64-bit only)

crates/db-server/src/
  lib.rs                 Tokio TCP accept skeleton

crates/db-sql/src/
  lib.rs                 sqlparser wrapper

crates/db-buffer/src/
  lib.rs                 (empty — scaffolding)

crates/db-mvcc/src/
  lib.rs                 (empty — scaffolding)

crates/db-optimizer/src/
  lib.rs                 (empty — scaffolding)

crates/db-scheduler/src/
  lib.rs                 (empty — scaffolding)

crates/db-wal/src/
  lib.rs                 (empty — scaffolding)
```
