use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::dtype::DataTypeKind;
use crate::error::{self, Result};
use crate::format_table::OutputTable;
use crate::table::{DBTable, TableId};
use dashmap::DashMap;
use dashmap::mapref::entry::Entry;
use snafu::prelude::*;

/// Named pair for defining a table column's name and type.
///
/// Avoids same-type `(&str, &str)` ambiguity at call sites.
pub struct FieldDef<'a> {
    pub name: &'a str,
    pub type_name: &'a str,
}

/// Top-level database owning all tables.
pub struct Database {
    tables: DashMap<String, Arc<DBTable>, ahash::RandomState>,
    table_id_next: AtomicU32,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: DashMap::with_hasher(ahash::RandomState::new()),
            table_id_next: AtomicU32::new(0),
        }
    }

    /// Create a new table with the given field definitions.
    pub fn create_table(&self, table_name: &str, fields: &[FieldDef<'_>]) -> Result<()> {
        match self.tables.entry(table_name.to_owned()) {
            Entry::Occupied(_) => error::TableAlreadyExistsSnafu { table_name }.fail(),
            Entry::Vacant(entry) => {
                let mut field_names = Vec::with_capacity(fields.len());
                let mut data_types = Vec::with_capacity(fields.len());

                for field in fields {
                    field_names.push(field.name.to_owned());
                    data_types.push(DataTypeKind::parse(field.type_name)?);
                }

                let table = Arc::new(DBTable::new(
                    self.next_table_id(),
                    entry.key().clone(),
                    field_names,
                    data_types,
                ));

                entry.insert(table);
                Ok(())
            }
        }
    }

    /// Insert tightly-packed row bytes into a named table.
    pub fn insert(&self, table_name: &str, bytes: &[u8]) -> Result<()> {
        let table = self.table(table_name)?;
        table.insert(bytes)
    }

    /// Return a formatted text representation of all rows in a table.
    pub fn query_all(&self, table_name: &str) -> Result<OutputTable> {
        let table = self.table(table_name)?;
        OutputTable::from_table(&table)
    }

    fn next_table_id(&self) -> TableId {
        let table_id = self.table_id_next.fetch_add(1, Ordering::Relaxed);
        TableId::new(table_id)
    }

    fn table(&self, table_name: &str) -> Result<Arc<DBTable>> {
        self.tables
            .get(table_name)
            .map(|table| Arc::clone(table.value()))
            .context(error::TableNotFoundSnafu { table_name })
    }
}
