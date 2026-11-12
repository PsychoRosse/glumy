#[derive(Debug, Clone)]
pub struct BindGroupLayout {
    layout: wgpu::BindGroupLayout,
}

impl BindGroupLayout {
    pub(crate) fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }
}
