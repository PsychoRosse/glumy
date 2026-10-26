use crate::TextureView;

pub struct SurfaceTexture {
    texture: wgpu::SurfaceTexture,
}

impl SurfaceTexture {
    pub(crate) fn new(texture: wgpu::SurfaceTexture) -> Self {
        Self { texture }
    }

    pub(crate) fn get(&self) -> &wgpu::SurfaceTexture {
        &self.texture
    }

    pub(crate) fn take(self) -> wgpu::SurfaceTexture {
        self.texture
    }

    pub(crate) fn create_view(texture: SurfaceTexture) -> TextureView {
        TextureView::new(
            texture
                .get()
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        )
    }
}
