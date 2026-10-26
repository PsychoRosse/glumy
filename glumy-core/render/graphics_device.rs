use crate::{Adapter, CommandEncoder, InstanceError, SurfaceTexture};

use {pollster::block_on, std::iter::once};

#[cfg(feature = "clama")]
use clama::Resource;

#[cfg_attr(feature = "clama", derive(Resource))]
pub struct GraphicsDevice {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl GraphicsDevice {
    pub fn new(adapter: &Adapter) -> Self {
        let (device, queue) = block_on(adapter.get().request_device(&wgpu::DeviceDescriptor {
            label: Some("Glumy Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }))
        .unwrap_or_else(|_| panic!("{}", InstanceError::DeviceCreate));

        Self { device, queue }
    }
}

impl GraphicsDevice {
    pub fn create_encoder(&self) -> CommandEncoder {
        CommandEncoder::new(
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default()),
        )
    }

    pub fn present(&self, encoder: CommandEncoder, frame: SurfaceTexture) {
        self.queue.submit(once(encoder.take().finish()));
        self.queue.present(frame.take())
    }
}

impl GraphicsDevice {
    pub(crate) fn get(&self) -> &wgpu::Device {
        &self.device
    }
}
