use std::collections::HashMap;

use crate::dtype::DataTypeKind;
use crate::format_table::OutputTable;
use crate::table::DBTable;

pub struct Database {
    tables: HashMap<String, DBTable>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, table_name: &str, fields: &[(&str, &str)]) {
        if self.tables.contains_key(table_name) {
            panic!("table already exists: {table_name}");
        }

        let mut field_names = Vec::with_capacity(fields.len());
        let mut data_types  = Vec::with_capacity(fields.len());

        for field in fields {
            field_names.push(field.0.to_owned());
            data_types.push(DataTypeKind::parse(field.1));
        }

        let table_name = table_name.to_owned();

        let table = DBTable::new(
            table_name.clone(),
            field_names,
            data_types,
        );

        self.tables.insert(table_name, table);
    }

    pub fn insert(&mut self, table_name: &str, bytes: &[u8]) {
        let table = self.table_mut(table_name);
        table.insert(bytes)
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
