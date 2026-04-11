use bytemuck::{Pod, Zeroable};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use db_catalog::Database;
use db_execution::execute;
use db_types::{PhysicalPlan, OutputTable};

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

const ROW_COUNT: usize = 100_000;
const TABLE_NAME: &str = "vec3";

fn loaded_database() -> Database {
    let mut db = Database::new();
    db.create_table(TABLE_NAME, &[("x", "f32"), ("y", "f32"), ("z", "f32")]);

    let points: Vec<Vec3> = (0..ROW_COUNT)
        .map(|i| {
            let t = i as f32;
            Vec3 { x: t, y: t * 2.0, z: t * 3.0 }
        })
        .collect();

    db.insert(TABLE_NAME, bytemuck::cast_slice(&points));
    db.table_mut(TABLE_NAME).flush_write_buffer();
    db
}

fn scan_plan() -> PhysicalPlan {
    PhysicalPlan::default()
}

fn bench_query(c: &mut Criterion) {
    let db = loaded_database();
    let plan = scan_plan();

    let mut group = c.benchmark_group("table_scan");
    group.throughput(Throughput::Elements(ROW_COUNT as u64));
    group.sample_size(20);

    group.bench_function("execute", |b| {
        b.iter(|| execute(plan.clone(), &db));
    });

    group.bench_function("execute_with_output", |b| {
        b.iter(|| OutputTable::from_query_result(&execute(plan.clone(), &db)));
    });

    group.finish();
}

criterion_group!(benches, bench_query);
criterion_main!(benches);
