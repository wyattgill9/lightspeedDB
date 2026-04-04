# Architecture

A columnar analytical database engine written in Rust (Edition 2024).
In-memory, single-writer, designed for high-throughput scan workloads.

## Crate Layout

```
db/
├── crates/
│   ├── db-core        Storage engine, schema, CLI entry point
│   ├── db-server      TCP server skeleton (Tokio + socket2)
│   └── db-sql         SQL parser wrapper (sqlparser)
├── CLAUDE.md          Project invariants and coding standards
└── ARCHITECTURE.md    This file
```

`db-core` is the default workspace member and the only crate with
substantial logic today. `db-server` and `db-sql` are scaffolding
for future network and query layers.

## Storage Model

The engine stores data **column-major**. Rows arrive as tightly-packed
byte arrays (array-of-structs), get buffered, then transposed into
per-column byte vectors inside immutable row groups.

```
Database
 └─ tables: HashMap<String, DBTable>      (rapidhash::fast)
     └─ DBTable
         ├─ write_buffer: Vec<u8>          row-major staging area
         ├─ row_groups: Vec<TablePartition> column-major partitions
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
4. Rows are split across partitions. If the active partition is full,
   a new one is allocated.

### TablePartition (Row Group)

Each partition holds up to **131 072 rows** (64 × 2 048). It contains
one `ColumnSegment` per schema column. The partition tracks its own
`row_count` and derives `rows_available` from the fixed capacity.

### Column-Major Transposition

`TablePartition::insert_rows` iterates **columns outer, rows inner**.
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

`stringt.rs` contains an unconnected design for variable-length
strings:

- **StringRef**: 16-byte aligned. 4-byte length + 12-byte payload.
  Strings ≤ 12 bytes are inlined; longer strings store a 4-byte
  prefix plus an arena index and offset.
- **StringBuffer**: Per-segment arena backed by a linked list of
  1 MB `ArenaBuffer` blocks.

## Schema

```
TableSchema
 └─ columns: Vec<ColumnDef>
 └─ row_size_bytes: usize        sum of column widths

ColumnDef
 └─ data_type: DataTypeKind
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

## Platform Requirements

`main.rs` is gated on `#[cfg(target_pointer_width = "64")]`. On
32-bit targets the binary prints an error and exits. The engine relies
on 64-bit pointer arithmetic for segment indexing and makes no
provision for 32-bit address spaces.

## Query Layer

Currently minimal:

- **db-sql** wraps `sqlparser` (v0.61) to parse SQL strings into an
  AST and returns the debug representation.
- **Database::execute_query** calls the parser but does not evaluate.
- **Database::print_table** performs a full scan: flush the write
  buffer, iterate all partitions, reconstruct rows from columnar data,
  and render an ASCII table via `OutputTable`.

No execution operators, predicate evaluation, or aggregation exist
yet. The scan path in `OutputTable::from_table` demonstrates the
read-side column-to-row reconstruction.

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

`crates/db-core/benches/insert.rs` measures insert throughput with
Criterion:

- **vec3_one_at_a_time**: 100 000 rows inserted individually.
- **vec3_bulk**: 100 000 rows inserted in a single `&[u8]` slice.

Both use a `Vec3 { x: f32, y: f32, z: f32 }` schema. Sample size is
20 iterations. Throughput is reported in elements/sec.

## Design Principles

Drawn from `CLAUDE.md`:

- **Safety > performance > developer experience.**
- **Correctness > compatibility.**
- Column-major layout for cache-friendly scans.
- Zero-copy by default (`bytemuck::cast_slice`, `&[u8]` slices).
- Bound everything: write buffer capped at 4 096 rows, partitions at
  131 072 rows, table count validated against `u32::MAX`.
- Fixed-width types only (no variable-length allocation on the hot
  path today).
- Explicit control flow, no recursion, every `if` has an `else`.
- Edition 2024, nightly Rust.

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
                │  write_rows_to_       │  split across partitions,
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
crates/db-core/src/
  main.rs              CLI entry point, example usage (64-bit only)
  lib.rs               Module declarations
  db.rs                Database: table map, insert, query, print
  table.rs             DBTable: write buffer, flush, partition management
  table_partition.rs   TablePartition: column-major transposition (row group)
  column_segment.rs    ColumnSegment: dense bytes + ZoneMap + BloomFilter + HLL
  table_schema.rs      TableSchema: column definitions, row width
  column_def.rs        ColumnDef: name, type, width
  dtype.rs             DataTypeKind: type enum, parse, format
  table_meta.rs        TableMeta: name + id
  table_stats.rs       TableStatistics (placeholder)
  zone_map.rs          ZoneMap: type-aware min/max tracking per segment
  stringt.rs           String arena design (not connected)
  table_format.rs      OutputTable: ASCII table renderer

crates/db-core/benches/
  insert.rs            Criterion insert benchmarks

crates/db-server/src/
  lib.rs               Tokio TCP accept skeleton

crates/db-sql/src/
  lib.rs               sqlparser wrapper
```
