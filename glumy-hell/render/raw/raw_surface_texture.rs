use crate::{TextureView, TextureViewDescriptor};

#[derive(Debug, Clone)]
pub struct RawSurfaceTexture {
    raw: wgpu::SurfaceTexture,
}

impl RawSurfaceTexture {
    pub fn create_view(&self, descriptor: TextureViewDescriptor) -> TextureView {
        TextureView::from_wgpu(self.raw.texture.create_view(&descriptor.to_wgpu()))
    }

    pub fn create_default_view(&self) -> TextureView {
        TextureView::from_wgpu(
            self.raw
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        )
    }

    pub fn raw(&self) -> &wgpu::SurfaceTexture {
        &self.raw
    }
}

impl RawSurfaceTexture {
    pub(crate) fn from_wgpu(raw: wgpu::SurfaceTexture) -> Self {
        Self { raw }
    }

    pub(crate) fn to_wgpu(&self) -> &wgpu::SurfaceTexture {
        &self.raw
    }

    pub(crate) fn raw_take(self) -> wgpu::SurfaceTexture {
        self.raw
    }
}
