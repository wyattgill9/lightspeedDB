use crate::error::{self, Result};

/// Supported column data types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataTypeKind {
    U64,
    U32,
    U8,
    I64,
    I32,
    I8,
    F32,
    F64,
    BOOL,
}

impl DataTypeKind {
    /// Parse a type name string (case-insensitive) into a `DataTypeKind`.
    pub fn parse(type_name: &str) -> Result<Self> {
        let upper = type_name.to_uppercase();
        let kind = match upper.as_str() {
            "U64" => DataTypeKind::U64,
            "U32" => DataTypeKind::U32,
            "U8" => DataTypeKind::U8,
            "I64" => DataTypeKind::I64,
            "I32" => DataTypeKind::I32,
            "I8" => DataTypeKind::I8,
            "F32" => DataTypeKind::F32,
            "F64" => DataTypeKind::F64,
            "BOOL" => DataTypeKind::BOOL,
            _ => return error::UnknownDataTypeSnafu { type_name }.fail(),
        };
        Ok(kind)
    }

    /// Byte width of a single value of this type.
    pub fn byte_width(self) -> usize {
        match self {
            DataTypeKind::U64 | DataTypeKind::I64 | DataTypeKind::F64 => 8,
            DataTypeKind::U32 | DataTypeKind::I32 | DataTypeKind::F32 => 4,
            DataTypeKind::U8 | DataTypeKind::I8 | DataTypeKind::BOOL => 1,
        }
    }

    /// Format raw little-endian bytes as a human-readable string.
    ///
    /// Caller must pass exactly `self.byte_width()` bytes.
    pub fn format_bytes(self, bytes: &[u8]) -> String {
        match self {
            DataTypeKind::U64 => format!("{}", u64::from_le_bytes(bytes.try_into().unwrap())),
            DataTypeKind::U32 => format!("{}", u32::from_le_bytes(bytes.try_into().unwrap())),
            DataTypeKind::U8 => format!("{}", bytes[0]),
            DataTypeKind::I64 => format!("{}", i64::from_le_bytes(bytes.try_into().unwrap())),
            DataTypeKind::I32 => format!("{}", i32::from_le_bytes(bytes.try_into().unwrap())),
            DataTypeKind::I8 => format!("{}", bytes[0] as i8),
            DataTypeKind::F32 => format!("{:.6}", f32::from_le_bytes(bytes.try_into().unwrap())),
            DataTypeKind::F64 => format!("{:.6}", f64::from_le_bytes(bytes.try_into().unwrap())),
            DataTypeKind::BOOL => {
                if bytes[0] != 0 {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
        }
    }
}
