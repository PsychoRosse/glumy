pub struct TextureView {
    view: wgpu::TextureView,
}

impl TextureView {
    pub(crate) fn new(view: wgpu::TextureView) -> Self {
        Self { view }
    }

    pub(crate) fn get(&self) -> &wgpu::TextureView {
        &self.view
    }
}
