use crate::{CommandEncoder, GlumyHellError, GpuInstance, SurfaceTexture, WindowSurface};

use {clama::Resource, pollster::block_on, std::iter::once};

#[derive(Resource, Debug)]
pub struct GraphicsDevice {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl GraphicsDevice {
    pub fn new(instance: &GpuInstance, surface: &WindowSurface) -> Result<Self, GlumyHellError> {
        let adapter = block_on(
            instance
                .get()
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    force_fallback_adapter: false,
                    compatible_surface: Some(surface.get()),
                    apply_limit_buckets: false,
                }),
        )
        .map_err(|_| GlumyHellError::AdapterRequest)?;

        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("Glumy Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }))
        .map_err(|_| GlumyHellError::DeviceCreate)?;

        Ok(Self {
            adapter,
            device,
            queue,
        })
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

    pub(crate) fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub(crate) fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }
}
