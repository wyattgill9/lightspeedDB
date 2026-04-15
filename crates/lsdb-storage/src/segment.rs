use cardinality_estimator::CardinalityEstimator;
use fastbloom::BloomFilter;

use lsdb_types::{ColumnDefinition, TableSchema};

use crate::zone_map::ZoneMap;

#[derive(Debug)]
pub struct ColumnSegment {
    data: Vec<u8>,
    column_def_index: usize,
    zone_map: ZoneMap,
    bloom: Option<BloomFilter<rapidhash::quality::RandomState>>,
    hll: CardinalityEstimator<[u8], rapidhash::quality::RapidHasher<'static>>,
}

impl ColumnSegment {
    pub fn new(column_index: usize) -> Self {
        Self {
            data: Vec::new(),
            column_def_index: column_index,
            zone_map: ZoneMap::new(),
            bloom: Some(
                BloomFilter::with_num_bits(64)
                    .hasher(rapidhash::quality::RandomState::default())
                    .hashes(1),
            ),
            hll: CardinalityEstimator::new(),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    pub fn push_dtype_val(&mut self, bytes: &[u8], schema: &TableSchema) {
        if let Some(bloom_filter) = &mut self.bloom {
            bloom_filter.insert(bytes);
        }
        self.hll.insert(bytes);
        self.zone_map
            .update(bytes, self.definition(schema).data_type());
        self.data.extend_from_slice(bytes);
    }

    pub fn definition<'a>(&self, schema: &'a TableSchema) -> &'a ColumnDefinition {
        schema.column_at(self.column_def_index)
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
