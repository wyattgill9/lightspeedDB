use db_types::{LogicalPlan, PhysicalPlan};

/// Convert a logical plan into a physical plan.
///
/// Phase 1: direct passthrough. No rewrites, no cost model.
pub fn plan(logical: &LogicalPlan) -> PhysicalPlan {
    match logical {
        LogicalPlan::Scan {
            table_name,
            column_indices,
        } => PhysicalPlan::TableScan {
            table_name: table_name.clone(),
            column_indices: column_indices.clone(),
        },
    }
}
