use std::collections::HashMap;

use crate::table::DBTable;
use crate::table_format::OutputTable;
use crate::table_schema::TableSchema;

pub struct Database {
    tables: HashMap<String, DBTable, rapidhash::fast::RandomState>,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::with_hasher(rapidhash::fast::RandomState::default()),
        }
    }

    pub fn create_table_with_schema(&mut self, table_name: &str, schema: TableSchema) {
        if self.tables.contains_key(table_name) {
            panic!("table already exists: {table_name}");
        } else {
            let table_name = table_name.to_owned();
            let id = u32::try_from(self.tables.len())
                .unwrap_or_else(|_| panic!("table count exceeds u32::MAX"));
            let table = DBTable::new(table_name.clone(), id, schema);
            self.tables.insert(table_name, table);
        }
    }

    pub fn create_table(&mut self, table_name: &str, fields: &[(&str, &str)]) {
        self.create_table_with_schema(table_name, TableSchema::from_fields(fields));
    }

    pub fn insert(&mut self, table_name: &str, bytes: &[u8]) {
        let table = self.table_mut(table_name);
        table.insert(bytes)
    }

    pub fn execute_query(self, query_str: &str) -> String {
        db_sqlparser::parse(query_str)
    }

    pub fn print_table(&mut self, table_name: &str) -> OutputTable {
        let table = self
            .tables
            .get_mut(table_name)
            .unwrap_or_else(|| panic!("table not found: {table_name}"));

        table.flush_write_buffer();
        OutputTable::from_table(table)
    }

    fn table_mut(&mut self, table_name: &str) -> &mut DBTable {
        self.tables
            .get_mut(table_name)
            .unwrap_or_else(|| panic!("table not found: {table_name}"))
    }
}
