pub trait ExecutionVector {
    fn len(&self) -> usize;

    fn as_any(&self) -> &dyn std::any::Any;
}
