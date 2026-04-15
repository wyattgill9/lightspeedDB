use lsdb_catalog::Database;
use lsdb_types::{ResolvedPlan, UnresolvedPlan};

pub fn bind(_plan: UnresolvedPlan, _database: &Database) -> ResolvedPlan {
    ResolvedPlan::default()
}
