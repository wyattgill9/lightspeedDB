use crate::exec::ExecutionVector;

pub struct FlatVector<T> {
    values: Vec<T>,
}

impl<T: 'static> ExecutionVector for FlatVector<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}
