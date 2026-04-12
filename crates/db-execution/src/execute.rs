use db_catalog::Database;
use db_types::{PhysicalPlan, QueryResult};

pub fn execute(_pplan: PhysicalPlan, _database: &Database) -> QueryResult {
    QueryResult::default()
}
