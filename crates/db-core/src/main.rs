#![cfg_attr(rustfmt, rustfmt_skip)]

use bytemuck::{Pod, Zeroable};

use db_core::db::Database;

fn main() {
    #[repr(C)]
    #[derive(Clone, Copy, Zeroable, Pod)]
    struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    let points = vec![
        Vec3 { x: 1.0,  y: 2.0,  z: 3.0,   },
        Vec3 { x: 4.0,  y: 5.0,  z: 6.0,   },
        Vec3 { x: 7.0,  y: 8.0,  z: 9.0,   },
        Vec3 { x: -1.5, y: 0.25, z: 100.0, },
    ];
    let point_bytes: &[u8] = bytemuck::cast_slice(&points);

    let mut db = Database::new();

    db.create_table("vec3", &[
        ("x", "f32"),
        ("y", "f32"),
        ("z", "f32"),
    ]);

    db.insert("vec3", point_bytes);

    let result = db.print_table("vec3"); // db.execute_query("SELECT * FROM vec3");
    println!("{result}");
}
