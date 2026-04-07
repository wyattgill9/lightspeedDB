pub enum LogicalPlan {
    Scan {
        table_name: String,
        column_indices: Vec<usize>,
    },
}
