use cardinality_estimator::CardinalityEstimator;
use fastbloom::BloomFilter;
use lsdb_types::TableSchema;

use crate::ZoneMap;

#[derive(Debug)]
pub struct ColumnSegmentStatistics {
    zone_map: ZoneMap,
    bloom: Option<BloomFilter<rapidhash::quality::RandomState>>,
    hll: CardinalityEstimator<[u8], rapidhash::quality::RapidHasher<'static>>,
}

impl ColumnSegmentStatistics {
    pub fn new() -> Self {
        Self {
            zone_map: ZoneMap::new(),
            bloom: Some(
                BloomFilter::with_num_bits(64)
                    .hasher(rapidhash::quality::RandomState::default())
                    .hashes(1),
            ),
            hll: CardinalityEstimator::new(),
        }
    }

    pub fn update(&mut self, bytes: &[u8], schema: &TableSchema, idx: usize) {
        if let Some(bloom_filter) = &mut self.bloom {
            bloom_filter.insert(bytes);
        }

        self.hll.insert(bytes);
        self.zone_map
            .update(bytes, schema.column_at(idx).data_type());
    }
}
