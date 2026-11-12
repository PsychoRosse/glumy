use crate::BufferUsages;

pub struct BufferDescriptor<'d> {
    pub label: &'d str,
    pub content: &'d [u8],
    pub usages: BufferUsages,
}
