pub struct CommandEncoder {
    encoder: wgpu::CommandEncoder,
}

impl CommandEncoder {
    pub(crate) fn new(encoder: wgpu::CommandEncoder) -> Self {
        Self { encoder }
    }

    pub(crate) fn get(&self) -> &wgpu::CommandEncoder {
        &self.encoder
    }

    pub(crate) fn get_mut(&mut self) -> &mut wgpu::CommandEncoder {
        &mut self.encoder
    }

    pub(crate) fn take(self) -> wgpu::CommandEncoder {
        self.encoder
    }
}
