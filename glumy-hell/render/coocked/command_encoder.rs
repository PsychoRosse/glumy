use crate::RawCommandEncoder;

pub struct CommandEncoder {
    raw: RawCommandEncoder,
}

impl CommandEncoder {
    pub fn from_raw(raw: RawCommandEncoder) -> Self {
        Self { raw }
    }

    pub fn raw_get(&self) -> &RawCommandEncoder {
        &self.raw
    }

    pub fn raw_get_mut(&mut self) -> &mut RawCommandEncoder {
        &mut self.raw
    }

    pub fn take(self) -> RawCommandEncoder {
        self.raw
    }
}

impl CommandEncoder {
    pub(crate) fn from_wgpu(raw: wgpu::CommandEncoder) -> Self {
        Self {
            raw: RawCommandEncoder::from_wgpu(raw),
        }
    }
}
