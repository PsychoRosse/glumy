use crate::BufferUsages;

#[derive(Debug, Clone)]
pub struct BufferIsBuild {
    pub label: String,
    pub usages: BufferUsages,
    pub content: Vec<u8>,
}
