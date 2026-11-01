use crate::BufferUsages;

pub struct BufferIsBuild {
    pub label: String,
    pub usages: BufferUsages,
    pub content: Vec<u8>,
}
