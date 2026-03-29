use bytemuck::{Pod, Zeroable};

use db::database::{Database, FieldDefinition};
use db::error::Error;

fn main() {
    match run() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), Error> {
    #[repr(C)]
    #[derive(Clone, Copy, Zeroable, Pod)]
    struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    let points = vec![
        Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        },
        Vec3 {
            x: 7.0,
            y: 8.0,
            z: 9.0,
        },
        Vec3 {
            x: -1.5,
            y: 0.25,
            z: 100.0,
        },
    ];
    let point_bytes: &[u8] = bytemuck::cast_slice(&points);

    let mut database = Database::new();

    database.create_table(
        "vec3",
        &[
            FieldDefinition {
                name: "x",
                type_name: "f32",
            },
            FieldDefinition {
                name: "y",
                type_name: "f32",
            },
            FieldDefinition {
                name: "z",
                type_name: "f32",
            },
        ],
    )?;

    database.insert("vec3", point_bytes)?;

    let result = database.query_all("vec3")?;
    assert!(
        !result.to_string().is_empty(),
        "Query result must not be empty."
    );
    println!("{result}");

    Ok(())
}
