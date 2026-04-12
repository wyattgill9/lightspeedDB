use db_catalog::Database;
use db_types::LogicalPlan;

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
fn main() {
    let mut db = Database::new();

    db.create_table("trips", &[
        ("cab_type", "u8"),
        ("passenger_count", "u8"),
        ("total_amount", "f64"),
    ]);

    let sql = "SELECT passenger_count, avg(total_amount) \
               FROM trips GROUP BY passenger_count";

    let logical_plan = build_logical_plan(sql, &db);
    println!("{logical_plan:#?}");
}

fn build_logical_plan(sql: &str, db: &Database) -> LogicalPlan {
    let parsed = db_sql::parse(sql);
    let unresolved = db_sql::translate(parsed);
    let resolved = db_sql::bind(unresolved, db);
    let logical = db_optimizer::build_plan(resolved);
    db_optimizer::optimize(logical)
}
