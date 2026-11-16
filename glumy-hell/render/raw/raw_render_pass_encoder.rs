pub struct RawRenderPassEncoder<'p> {
    raw: wgpu::RenderPass<'p>,
}

impl<'p> RawRenderPassEncoder<'p> {
    pub(crate) fn from_wgpu(raw: wgpu::RenderPass<'p>) -> Self {
        Self { raw }
    }

    pub(crate) fn raw_get_mut(&mut self) -> &mut wgpu::RenderPass<'p> {
        &mut self.raw
    }
}
