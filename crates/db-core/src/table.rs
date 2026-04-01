use crate::table_partition::TablePartition;
use crate::table_schema::TableSchema;
use crate::table_meta::TableMeta;
use crate::table_stats::TableStatistics;

#[derive(Debug)]
pub struct DBTable {
    meta: TableMeta,
    schema: TableSchema,
    row_groups: Vec<TablePartition>,
    stats: TableStatistics,
}

impl DBTable {
    pub fn new(name: String, schema: TableSchema) -> Self {
        let row_groups = vec![TablePartition::new(&schema)];

        Self {
            meta: TableMeta::new(name, 0), // TODO: 1 table
            schema,
            row_groups,
            stats: TableStatistics::new(),
        }
    }

    pub fn insert(&mut self, bytes: &[u8]) {
        let schema = &self.schema;

        if let Some(row_group) = self.row_groups.last_mut() {
            row_group.insert_rows(schema, bytes);
        } else {
            panic!("table has no row groups: {}", self.meta.name());
        }
    }

    pub fn name(&self) -> &str {
        self.meta.name()
    }

    pub fn id(&self) -> u32 {
        self.meta.id()
    }

    pub fn schema(&self) -> &TableSchema {
        &self.schema
    }

    pub fn row_groups(&self) -> &[TablePartition] {
        &self.row_groups
    }

    pub fn stats(&self) -> &TableStatistics {
        &self.stats
    }
}
