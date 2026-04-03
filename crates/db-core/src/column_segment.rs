use cardinality_estimator::CardinalityEstimator;

use crate::table_schema::TableSchema;
use crate::column_def::ColumnDef;
use crate::zone_map::ZoneMap;

#[derive(Debug)]
pub struct ColumnSegment {
    data: Vec<u8>,
    column_def_index: usize,
    _zone_map: ZoneMap,
    hll: CardinalityEstimator<[u8], rapidhash::quality::RapidHasher<'static>>, // HyperLogLog++
}

impl ColumnSegment {
    pub fn new(column_index: usize) -> Self {
        Self {
            data: Vec::new(),
            column_def_index: column_index,
            _zone_map: ZoneMap::new(),
            hll : CardinalityEstimator::new()
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }


    pub fn push_dtype_val(&mut self, bytes: &[u8]) {
        self.hll.insert(bytes);
        self.data.extend_from_slice(bytes);
    }

    pub fn def<'a>(&self, schema: &'a TableSchema) -> &'a ColumnDef {
        schema.column_at(self.column_def_index)
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
