use crate::QueryResult;

pub struct OutputTable {
    output: String,
}

impl std::fmt::Display for OutputTable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.output)
    }
}

impl OutputTable {
    pub fn from_query_result(_query_result: &QueryResult) -> Self {
        Self {
            output: "".to_string(),
        }
    }
}
