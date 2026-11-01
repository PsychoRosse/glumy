use crate::{BindingType, ShaderStages};

use std::num::NonZero;

pub struct BindGroupLayoitEntry {
    pub binding: u32,
    pub visibility: ShaderStages,
    pub ty: BindingType,
    pub count: Option<NonZero<u32>>,
}
