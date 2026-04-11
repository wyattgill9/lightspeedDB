#[derive(Default)]
pub enum UnresolvedPlan {
    Select {
        columns: Vec<String>,
    },

    #[default]
    None,
}

#[derive(Default)]
pub enum ResolvedPlan {
    #[default]
    None,
}

#[derive(Default)]
pub enum LogicalPlan {
    #[default]
    None,
}

#[derive(Default, Clone, Copy)]
pub enum PhysicalPlan {
    #[default]
    None,
}

