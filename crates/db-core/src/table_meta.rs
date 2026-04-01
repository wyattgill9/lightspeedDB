#[derive(Debug)]
pub struct TableMeta {
    name: String,
    id: u32,
}

impl TableMeta {
    pub fn new(name: String, id: u32) -> Self {
        Self { name, id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

