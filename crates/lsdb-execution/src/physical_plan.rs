use lsdb_types::{LogicalPlan, PhysicalPlan};

pub fn physical_plan(_lplan: LogicalPlan) -> PhysicalPlan {
    PhysicalPlan::default()
}
