use crate::BufferUsages;

#[derive(Debug, Clone)]
pub struct Buffer {
    label: String,
    content: Vec<u8>,
    usages: BufferUsages,
}

impl Buffer {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn usages(&self) -> BufferUsages {
        self.usages
    }
}
