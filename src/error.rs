use snafu::prelude::*;

/// Typed error enum for all fallible database operations.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("unknown data type: {type_name}"))]
    UnknownDataType { type_name: String },

    #[snafu(display("table not found: {table_name}"))]
    TableNotFound { table_name: String },

    #[snafu(display(
        "row bytes length {length_bytes_actual} is not a multiple of row size {size_bytes_row}"
    ))]
    InvalidRowBytes {
        length_bytes_actual: usize,
        size_bytes_row: usize,
    },

    #[snafu(display("table already exists: {table_name}"))]
    TableAlreadyExists { table_name: String },
}

/// Crate-wide result alias.
pub type Result<T> = std::result::Result<T, Error>;
