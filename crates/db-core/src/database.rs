use std::collections::HashMap;

use crate::dtype::DataTypeKind;
use crate::error::{self, Result};
use crate::format_table::OutputTable;
use crate::table::{DatabaseTable, TableId};
use snafu::prelude::*;

/// Named pair for defining a table column's name and type.
///
/// Avoids same-type `(&str, &str)` ambiguity at call sites.
pub struct FieldDefinition<'a> {
    pub name: &'a str,
    pub type_name: &'a str,
}

/// Top-level database owning all tables.
pub struct Database {
    tables: HashMap<String, DatabaseTable, ahash::RandomState>,
    table_id_next: TableId,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::with_hasher(ahash::RandomState::new()),
            table_id_next: TableId::new(0),
        }
    }

    /// Create a new table with the given field definitions.
    pub fn create_table(&mut self, table_name: &str, fields: &[FieldDefinition<'_>]) -> Result<()> {
        if self.tables.contains_key(table_name) {
            return error::TableAlreadyExistsSnafu { table_name }.fail();
        }

        let field_names: Vec<String> = fields.iter().map(|field| field.name.to_string()).collect();
        let data_types: Vec<DataTypeKind> = fields
            .iter()
            .map(|field| DataTypeKind::parse(field.type_name))
            .collect::<Result<Vec<_>>>()?;

        let table = DatabaseTable::new(
            self.table_id_next,
            table_name.to_string(),
            field_names,
            data_types,
        );

        self.tables.insert(table_name.to_string(), table);
        self.table_id_next = self.table_id_next.next();

        Ok(())
    }

    /// Insert tightly-packed row bytes into a named table.
    pub fn insert(&mut self, table_name: &str, bytes: &[u8]) -> Result<()> {
        let table = self
            .tables
            .get_mut(table_name)
            .context(error::TableNotFoundSnafu { table_name })?;

        table.insert(bytes)
    }

    /// Return a formatted text representation of all rows in a table.
    pub fn query_all(&self, table_name: &str) -> Result<OutputTable> {
        let table = self
            .tables
            .get(table_name)
            .context(error::TableNotFoundSnafu { table_name })?;

        Ok(OutputTable::from_table(table))
    }
}
