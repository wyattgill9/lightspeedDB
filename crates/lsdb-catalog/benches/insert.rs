use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use lsdb_catalog::DBTable;
use lsdb_types::TableSchema;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[repr(C)]
#[derive(Clone, Copy, FromBytes, IntoBytes, Immutable, KnownLayout)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[rustfmt::skip]
fn schema() -> TableSchema {
    TableSchema::from_fields(&[
        ("x", "f32"),
        ("y", "f32"),
        ("z", "f32")
    ])
}

fn fresh_table() -> DBTable {
    DBTable::new("bench".to_owned(), 0, schema())
}

#[rustfmt::skip]
fn points() -> Vec<Vec3> {
    (0..ROW_COUNT)
        .map(|i| {
            let t = i as f32;
            Vec3 { x: t, y: t * 2.0, z: t * 3.0, }
        })
        .collect()
}

const ROW_COUNT: usize = 1_00_000;

fn bench_insert(c: &mut Criterion) {
    let points = points();

    let mut group = c.benchmark_group("table_insert");
    group.throughput(Throughput::Elements(ROW_COUNT as u64));
    group.sample_size(20);

    group.bench_function("vec3_one_at_a_time", |b| {
        b.iter_batched(
            fresh_table,
            |mut table| {
                for point in &points {
                    table.insert(point.as_bytes());
                }
            },
            BatchSize::LargeInput,
        );
    });

    group.bench_function("vec3_bulk", |b| {
        b.iter_batched(
            fresh_table,
            |mut table| {
                table.insert(points.as_bytes());
            },
            BatchSize::LargeInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_insert);
criterion_main!(benches);
