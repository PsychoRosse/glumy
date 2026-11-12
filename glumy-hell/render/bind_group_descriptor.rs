use crate::{BindGroupEntry, BindGroupLayout};

pub struct BindGroupDescriptor<'b> {
    pub label: &'b str,
    pub layout: &'b BindGroupLayout,
    pub entries: &'b [BindGroupEntry<'b>],
}
