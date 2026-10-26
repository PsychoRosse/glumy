use crate::{Adapter, CommandEncoder, GpuInstance, InstanceError, SurfaceTexture, WindowSurface};

use std::iter::once;

pub struct GraphicsDevice {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl GraphicsDevice {
    pub async fn new(adapter: &Adapter) -> Self {
        let (device, queue) = adapter
            .get()
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Glumy Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::defaults(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
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
