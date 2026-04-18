use std::mem::size_of;

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use lsdb_types::DataTypeKind;

#[derive(Debug)]
pub struct ZoneMap {
    // Only works with fixed-size dtypes up to 8 bytes.
    min_bytes: [u8; 8],
    max_bytes: [u8; 8],
}

impl Default for ZoneMap {
    fn default() -> Self {
        Self::new()
    }
}

impl ZoneMap {
    pub fn new() -> Self {
        Self {
            min_bytes: [0; 8],
            max_bytes: [0; 8],
        }
    }

    pub fn get_max(&self) -> [u8; 8] {
        self.max_bytes
    }

    pub fn get_min(&self) -> [u8; 8] {
        self.min_bytes
    }

    pub fn update(&mut self, bytes: &[u8], dtype: DataTypeKind) {
        match dtype {
            DataTypeKind::U64 => self.compare_bytes::<u64>(&bytes[0..8]),
            DataTypeKind::U32 => self.compare_bytes::<u32>(&bytes[0..4]),
            DataTypeKind::U8 => self.compare_bytes::<u8>(&bytes[0..1]),
            DataTypeKind::I64 => self.compare_bytes::<i64>(&bytes[0..8]),
            DataTypeKind::I32 => self.compare_bytes::<i32>(&bytes[0..4]),
            DataTypeKind::I8 => self.compare_bytes::<i8>(&bytes[0..1]),
            DataTypeKind::F64 => self.compare_bytes::<f64>(&bytes[0..8]),
            DataTypeKind::F32 => self.compare_bytes::<f32>(&bytes[0..4]),
            DataTypeKind::BOOL => self.compare_bytes::<u8>(&bytes[0..1]),
        }
    }

    fn compare_bytes<T>(&mut self, bytes: &[u8])
    where
        T: FromBytes + IntoBytes + Immutable + KnownLayout + PartialOrd,
    {
        let size = size_of::<T>();
        let new: T =
            T::read_from_bytes(bytes).expect("byte slice length must equal size_of::<T>()");
        let cur_min: T = T::read_from_bytes(&self.min_bytes[..size])
            .expect("min_bytes slice length must equal size_of::<T>()");
        let cur_max: T = T::read_from_bytes(&self.max_bytes[..size])
            .expect("max_bytes slice length must equal size_of::<T>()");

        if new > cur_max {
            self.max_bytes[..size].copy_from_slice(new.as_bytes());
        }

        if new < cur_min {
            self.min_bytes[..size].copy_from_slice(new.as_bytes());
        }
    }
}
