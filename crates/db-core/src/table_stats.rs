#[derive(Debug)]
pub struct TableStatistics {}

impl Default for TableStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl TableStatistics {
    pub fn new() -> Self {
        Self {}
    }
}
