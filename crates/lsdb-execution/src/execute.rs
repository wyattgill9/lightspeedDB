use lsdb_catalog::Database;
use lsdb_types::{PhysicalPlan, QueryResult};

pub fn execute(_pplan: PhysicalPlan, _database: &Database) -> QueryResult {
    QueryResult::default()
}
