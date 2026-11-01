use std::num::NonZero;

use crate::BufferBindingType;

pub enum BindingType {
    Buffer {
        ty: BufferBindingType,
        has_dynamic_offset: bool,
        min_binding_size: Option<NonZero<u32>>,
    },
}
