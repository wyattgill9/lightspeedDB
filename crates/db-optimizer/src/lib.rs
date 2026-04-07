use db_types::{LogicalPlan, ResolvedPlan};

pub fn build_plan(_rplan: ResolvedPlan) -> LogicalPlan {
    LogicalPlan::default()
}

pub fn optimize(_lplan: LogicalPlan) -> LogicalPlan {
    LogicalPlan::default()
}
