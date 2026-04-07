#![cfg_attr(rustfmt, rustfmt_skip)]

use bytemuck::{Pod, Zeroable};

use db_catalog::Database;
use db_types::QueryResult;

#[cfg(target_pointer_width = "64")]
fn main() {
    #[repr(C)]
    #[derive(Clone, Copy, Zeroable, Pod)]
    struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    let points = vec![
        Vec3 { x: 1.0,  y: 2.0,  z: 3.0, },
        Vec3 { x: 4.0,  y: 5.0,  z: 6.0, },
        Vec3 { x: 7.0,  y: 8.0,  z: 9.0, },
        Vec3 { x: -1.5, y: 0.25, z: 1.0, },
    ];
    let point_bytes: &[u8] = bytemuck::cast_slice(&points);

    let mut db = Database::new();

    db.create_table("vec3", &[
        ("x", "f32"),
        ("y", "f32"),
        ("z", "f32"),
    ]);

    db.insert("vec3", point_bytes);
    db.flush_table_writes("vec3");

    let output = run_sql_query("SELECT col1 FROM vec3", &db);

    println!("{:?}", output);
}

fn run_sql_query(sql: &str, db: &Database) -> QueryResult {
    let parsed     = db_sql::parse(sql);
    let unresolved = db_sql::translate(parsed);
    let resolved   = db_sql::bind(unresolved, &db);
    let logical    = db_optimizer::build_plan(resolved);
    let optimized  = db_optimizer::optimize(logical);
    let physical   = db_execution::physical_plan(optimized);

    db_execution::execute(physical, &db)
}
