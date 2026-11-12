use crate::BindingResource;

pub struct BindGroupEntry<'e> {
    pub binding: u32,
    pub resource: BindingResource<'e>,
}
