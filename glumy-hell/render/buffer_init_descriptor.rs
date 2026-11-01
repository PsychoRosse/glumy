use crate::{BufferUsages, GraphicsDevice};

#[derive(Clone, Copy)]
pub struct BufferInitDescriptor<'bu> {
    pub device: &'bu GraphicsDevice,
    pub label: Option<&'bu str>,
    pub content: &'bu [u8],
    pub usages: BufferUsages,
}
