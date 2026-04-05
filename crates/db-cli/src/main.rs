#![cfg_attr(rustfmt, rustfmt_skip)]

use bytemuck::{Pod, Zeroable};

use db_catalog::Database;
use db_execution::OutputTable;

fn print_table(db: &mut Database, table_name: &str) -> OutputTable {
    let table = db.table_mut(table_name);
    table.flush_write_buffer();
    OutputTable::from_table(table)
}

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

    let result = print_table(&mut db, "vec3");
    println!("{result}");
}

#[cfg(not(target_pointer_width = "64"))]
fn main() {
    println!("This program requires a 64-bit target.");
}
