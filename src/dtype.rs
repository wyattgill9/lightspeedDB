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
    Bool,
}

impl DataTypeKind {
    /// Parse a type name string (case-insensitive) into a `DataTypeKind`.
    pub fn parse(type_name: &str) -> Result<Self> {
        assert!(!type_name.is_empty(), "Type name must not be empty.");
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
            "BOOL" => DataTypeKind::Bool,
            _ => return error::UnknownDataTypeSnafu { type_name }.fail(),
        };
        debug_assert!(
            kind.byte_width() > 0,
            "Parsed type must have positive byte width."
        );
        Ok(kind)
    }

    /// Byte width of a single value of this type.
    pub fn byte_width(self) -> usize {
        let width = match self {
            DataTypeKind::U64 | DataTypeKind::I64 | DataTypeKind::F64 => 8,
            DataTypeKind::U32 | DataTypeKind::I32 | DataTypeKind::F32 => 4,
            DataTypeKind::U8 | DataTypeKind::I8 | DataTypeKind::Bool => 1,
        };
        more_asserts::debug_assert_gt!(width, 0, "Byte width must be positive.");
        more_asserts::debug_assert_le!(width, 8, "Byte width must not exceed 8.");
        width
    }

    /// Format raw little-endian bytes as a human-readable string.
    ///
    /// Caller must pass exactly `self.byte_width()` bytes.
    pub fn format_bytes(self, bytes: &[u8]) -> String {
        assert_eq!(
            bytes.len(),
            self.byte_width(),
            "Byte slice length must equal type byte width."
        );
        let result = match self {
            DataTypeKind::U64 => {
                format!("{}", u64::from_le_bytes(bytes.try_into().unwrap()))
            }
            DataTypeKind::U32 => {
                format!("{}", u32::from_le_bytes(bytes.try_into().unwrap()))
            }
            DataTypeKind::U8 => format!("{}", bytes[0]),
            DataTypeKind::I64 => {
                format!("{}", i64::from_le_bytes(bytes.try_into().unwrap()))
            }
            DataTypeKind::I32 => {
                format!("{}", i32::from_le_bytes(bytes.try_into().unwrap()))
            }
            DataTypeKind::I8 => format!("{}", bytes[0] as i8),
            DataTypeKind::F32 => {
                format!("{:.6}", f32::from_le_bytes(bytes.try_into().unwrap()))
            }
            DataTypeKind::F64 => {
                format!("{:.6}", f64::from_le_bytes(bytes.try_into().unwrap()))
            }
            DataTypeKind::Bool => {
                if bytes[0] != 0 {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
        };
        debug_assert!(!result.is_empty(), "Formatted result must not be empty.");
        result
    }
}
