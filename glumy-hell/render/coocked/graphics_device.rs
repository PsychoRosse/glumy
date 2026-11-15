use crate::{CommandEncoder, RawGraphicsDevice, SurfaceTexture};

#[derive(Debug)]
pub struct GraphicsDevice {
    raw: RawGraphicsDevice,
}

impl GraphicsDevice {
    pub fn from_raw(raw: RawGraphicsDevice) -> Self {
        Self { raw }
    }

    pub fn raw(&self) -> &RawGraphicsDevice {
        &self.raw
    }

    pub fn create_encoder(&self) -> CommandEncoder {
        CommandEncoder::from_raw(self.raw.create_encoder())
    }

    pub fn present(&self, encoder: CommandEncoder, frame: SurfaceTexture) {
        self.raw.present(encoder.take(), frame.take());
    }
}

impl GraphicsDevice {}
