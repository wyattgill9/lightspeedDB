use std::collections::HashMap;

use crate::format_table::OutputTable;
use crate::table::DBTable;
use crate::table_schema::TableSchema;

pub struct Database {
    tables: HashMap<String, DBTable, rapidhash::fast::RandomState>,
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
            let table = DBTable::new(table_name.clone(), schema);
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

    pub fn print_table(&self, table_name: &str) -> OutputTable {
        let table = self.table(table_name);
        OutputTable::from_table(table)
    }

    fn table(&self, table_name: &str) -> &DBTable {
        self.tables
            .get(table_name)
            .unwrap_or_else(|| panic!("table not found: {table_name}"))
    }

    fn table_mut(&mut self, table_name: &str) -> &mut DBTable {
        self.tables
            .get_mut(table_name)
            .unwrap_or_else(|| panic!("table not found: {table_name}"))
    }
}
