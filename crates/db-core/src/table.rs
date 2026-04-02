use crate::table_meta::TableMeta;
use crate::table_partition::TablePartition;
use crate::table_schema::TableSchema;
use crate::table_stats::TableStatistics;

#[derive(Debug)]
pub struct DBTable {
    meta: TableMeta,
    schema: TableSchema,
    row_groups: Vec<TablePartition>,
    stats: TableStatistics,
}

impl DBTable {
    pub fn new(name: String, id: u32, schema: TableSchema) -> Self {
        let row_groups = vec![TablePartition::new(&schema)];

        Self {
            meta: TableMeta::new(name, id),
            schema,
            row_groups,
            stats: TableStatistics::new(),
        }
    }

    pub fn insert(&mut self, bytes: &[u8]) {
        let row_byte_width = self.schema.row_size_bytes();

        if !bytes.len().is_multiple_of(row_byte_width) {
            panic!(
                "byte length {} is not a multiple of row width {}",
                bytes.len(),
                row_byte_width
            );
        } else {
            let row_count_total = bytes.len() / row_byte_width;
            let mut row_count_done = 0usize;

            while row_count_done < row_count_total {
                let is_full = self
                    .row_groups
                    .last()
                    .is_none_or(|rg| rg.rows_available() == 0);

                if is_full {
                    self.row_groups.push(TablePartition::new(&self.schema));
                } else {
                    // current row group has capacity; no action needed
                }

                let schema = &self.schema;
                let row_group = self.row_groups.last_mut().expect("row group exists");

                let row_count_chunk =
                    (row_count_total - row_count_done).min(row_group.rows_available() as usize);
                let byte_start = row_count_done * row_byte_width;
                let byte_end = byte_start + row_count_chunk * row_byte_width;
                row_group.insert_rows(schema, &bytes[byte_start..byte_end]);
                row_count_done += row_count_chunk;
            }
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
