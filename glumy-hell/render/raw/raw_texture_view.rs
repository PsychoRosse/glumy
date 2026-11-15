pub struct RawTextureView {
    raw: wgpu::TextureView,
}

impl RawTextureView {
    pub(crate) fn raw_get(&self) -> &wgpu::TextureView {
        &self.raw
    }
}

impl RawTextureView {
    pub(crate) fn from_wgpu(raw: wgpu::TextureView) -> Self {
        Self { raw }
    }
}
