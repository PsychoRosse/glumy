pub struct RawCommandEncoder {
    raw: wgpu::CommandEncoder,
}

impl RawCommandEncoder {
    pub fn raw_get(&self) -> &wgpu::CommandEncoder {
        &self.raw
    }

    pub fn raw_get_mut(&mut self) -> &mut wgpu::CommandEncoder {
        &mut self.raw
    }
}

impl RawCommandEncoder {
    pub(crate) fn from_wgpu(raw: wgpu::CommandEncoder) -> Self {
        Self { raw }
    }

    pub(crate) fn take(self) -> wgpu::CommandEncoder {
        self.raw
    }
}
