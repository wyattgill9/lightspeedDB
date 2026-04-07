#[derive(Default)]
pub struct UnresolvedPlan {}

#[derive(Default)]
pub struct ResolvedPlan {}

#[derive(Default)]
pub enum LogicalPlan {
    #[default]
    None,
}

#[derive(Default)]
pub enum PhysicalPlan {
    #[default]
    None,
}

