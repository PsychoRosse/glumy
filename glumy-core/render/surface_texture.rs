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
}
