use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::dtype::DataTypeKind;
use crate::error::{self, Result};
use crate::format_table::OutputTable;
use crate::table::{DatabaseTable, TableId};
use dashmap::DashMap;
use dashmap::mapref::entry::Entry;
use snafu::prelude::*;

/// Named pair for defining a table column's name and type.
///
/// Avoids same-type `(&str, &str)` ambiguity at call sites.
pub struct FieldDefinition<'a> {
    pub name: &'a str,
    pub type_name: &'a str,
}

/// Shared concurrent table registry for worker fan-out.
pub type Tables = DashMap<String, Arc<DatabaseTable>, ahash::RandomState>;

/// Top-level database owning all tables.
pub struct Database {
    tables: Arc<Tables>,
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
            tables: Arc::new(DashMap::with_hasher(ahash::RandomState::new())),
            table_id_next: AtomicU32::new(0),
        }
    }

    pub fn tables_shared(&self) -> Arc<Tables> {
        Arc::clone(&self.tables)
    }

    /// Create a new table with the given field definitions.
    pub fn create_table(&self, table_name: &str, fields: &[FieldDefinition<'_>]) -> Result<()> {
        let field_names: Vec<String> = fields.iter().map(|field| field.name.to_string()).collect();
        let data_types: Vec<DataTypeKind> = fields
            .iter()
            .map(|field| DataTypeKind::parse(field.type_name))
            .collect::<Result<Vec<_>>>()?;

        match self.tables.entry(table_name.to_string()) {
            Entry::Occupied(_) => error::TableAlreadyExistsSnafu { table_name }.fail(),
            Entry::Vacant(entry) => {
                let table = Arc::new(DatabaseTable::new(
                    self.next_table_id(),
                    table_name.to_string(),
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
        // The counter only provides unique ids; it does not publish other state.
        let table_id = self.table_id_next.fetch_add(1, Ordering::Relaxed);
        TableId::new(table_id)
    }

    fn table(&self, table_name: &str) -> Result<Arc<DatabaseTable>> {
        self.tables
            .get(table_name)
            .map(|table| Arc::clone(table.value()))
            .context(error::TableNotFoundSnafu { table_name })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use bytemuck::{Pod, Zeroable};

    use super::{Database, FieldDefinition, Tables};
    use crate::error::Result;
    use crate::table::DatabaseTable;

    #[repr(C)]
    #[derive(Clone, Copy, Zeroable, Pod)]
    struct U32Row {
        value: u32,
    }

    fn assert_send_sync<T: Send + Sync>() {}

    fn build_rows(value_start: u32, count: u32) -> Vec<U32Row> {
        (0..count)
            .map(|offset| U32Row {
                value: value_start + offset,
            })
            .collect()
    }

    fn join_handles(handles: Vec<thread::JoinHandle<Result<()>>>) -> Result<()> {
        for handle in handles {
            match handle.join() {
                Ok(result) => result?,
                Err(payload) => std::panic::resume_unwind(payload),
            }
        }

        Ok(())
    }

    #[test]
    fn database_types_are_send_sync() {
        assert_send_sync::<Database>();
        assert_send_sync::<DatabaseTable>();
        assert_send_sync::<Tables>();
    }

    #[test]
    fn concurrent_inserts_share_database() -> Result<()> {
        let database = Arc::new(Database::new());
        let field_definitions = [FieldDefinition {
            name: "value",
            type_name: "u32",
        }];
        database.create_table("numbers", &field_definitions)?;

        let thread_count = 4;
        let rows_per_thread = 8;
        let mut handles = Vec::new();

        for thread_index in 0..thread_count {
            let database = Arc::clone(&database);
            handles.push(thread::spawn(move || -> Result<()> {
                let value_start = thread_index * rows_per_thread;
                let rows = build_rows(value_start, rows_per_thread);
                let bytes = bytemuck::cast_slice(&rows);
                database.insert("numbers", bytes)
            }));
        }

        join_handles(handles)?;

        let row_count_actual: u32 = database
            .table("numbers")?
            .row_groups_snapshot()?
            .iter()
            .map(|segment| segment.row_count())
            .sum();
        assert_eq!(row_count_actual, thread_count * rows_per_thread);

        let output = database.query_all("numbers")?;
        assert!(output.to_string().contains("Table: numbers"));

        Ok(())
    }
}
