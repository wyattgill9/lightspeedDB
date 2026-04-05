#![cfg_attr(rustfmt, rustfmt_skip)]

use bytemuck::{Pod, Zeroable};

use db_catalog::Database;
use db_execution::output::OutputTable;

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
    db.table_mut("vec3").flush_write_buffer();

    let logical  = db_sql::bind("SELECT * FROM vec3", &db);
    let physical = db_optimizer::plan(&logical);
    let result   = db_execution::execute::execute(&physical, &db);
    let output   = OutputTable::from_query_result(&result);
    println!("{output}");
}

#[cfg(not(target_pointer_width = "64"))]
fn main() {
    println!("This program requires a 64-bit target.");
}
