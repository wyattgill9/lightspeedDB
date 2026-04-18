use crate::exec::{ExecutionVector, FlatVector};

pub struct DataChunk {
    chunks: Vec<Box<dyn ExecutionVector>>,
}

impl DataChunk {
    pub fn column<T: 'static>(&self, idx: usize) -> Option<&FlatVector<T>> {
        self.chunks[idx].as_any().downcast_ref::<FlatVector<T>>()
    }
}
