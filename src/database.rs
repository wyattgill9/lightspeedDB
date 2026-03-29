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
        let database = Self {
            tables: HashMap::with_hasher(ahash::RandomState::new()),
            table_id_next: TableId::new(0),
        };
        debug_assert!(
            database.tables.is_empty(),
            "New database must have no tables."
        );
        database
    }

    /// Create a new table with the given field definitions.
    pub fn create_table(&mut self, table_name: &str, fields: &[FieldDefinition<'_>]) -> Result<()> {
        assert!(!table_name.is_empty(), "Table name must not be empty.");
        assert!(!fields.is_empty(), "Fields must not be empty.");

        if self.tables.contains_key(table_name) {
            return error::TableAlreadyExistsSnafu { table_name }.fail();
        }
        // else: table name is available, proceed.

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

        debug_assert!(
            self.tables.contains_key(table_name),
            "Table must exist after creation."
        );
        Ok(())
    }

    /// Insert tightly-packed row bytes into a named table.
    pub fn insert(&mut self, table_name: &str, bytes: &[u8]) -> Result<()> {
        assert!(!table_name.is_empty(), "Table name must not be empty.");
        assert!(!bytes.is_empty(), "Insert data must not be empty.");

        let table = self
            .tables
            .get_mut(table_name)
            .context(error::TableNotFoundSnafu { table_name })?;
        table.insert(bytes)
    }

    /// Return a formatted text representation of all rows in a table.
    pub fn query_all(&self, table_name: &str) -> Result<OutputTable> {
        assert!(!table_name.is_empty(), "Table name must not be empty.");

        let table = self
            .tables
            .get(table_name)
            .context(error::TableNotFoundSnafu { table_name })?;
        debug_assert!(
            !table.field_names().is_empty(),
            "Queried table must have fields."
        );
        Ok(OutputTable::from_table(table))
    }
}
