use db_catalog::Database;
use db_types::{LogicalPlan, PhysicalPlan, QueryResult};

pub fn physical_plan(_lplan: LogicalPlan) -> PhysicalPlan {
    PhysicalPlan::default()
}

pub fn execute(_pplan: PhysicalPlan, _database: &Database) -> QueryResult {
    QueryResult::default()
}
