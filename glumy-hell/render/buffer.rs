use crate::BufferUsages;

pub struct Buffer {
    label: String,
    content: Vec<u8>,
    usages: BufferUsages,
}

impl Buffer {
    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn content(&self) -> &Vec<u8> {
        &self.content
    }

    pub fn usages(&self) -> BufferUsages {
        self.usages
    }
}
