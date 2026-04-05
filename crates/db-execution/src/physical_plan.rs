pub enum PhysicalPlan {
    TableScan {
        table_name: String,
        column_indices: Vec<usize>,
    },
}
