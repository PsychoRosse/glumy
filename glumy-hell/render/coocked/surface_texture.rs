use crate::{RawSurfaceTexture, TextureView, TextureViewDescriptor};

pub struct SurfaceTexture {
    raw: RawSurfaceTexture,
}

impl SurfaceTexture {
    pub fn from_raw(raw: RawSurfaceTexture) -> Self {
        Self { raw }
    }

    pub fn create_view(&self, descriptor: TextureViewDescriptor) -> TextureView {
        self.raw.create_view(descriptor)
    }

    pub fn create_default_view(&self) -> TextureView {
        self.raw.create_default_view()
    }

    pub fn take(self) -> RawSurfaceTexture {
        self.raw
    }
}

impl SurfaceTexture {
    pub(crate) fn from_wgpu(raw: wgpu::SurfaceTexture) -> Self {
        Self {
            raw: RawSurfaceTexture::from_wgpu(raw),
        }
    }
}
