use crate::{HellError, RawCommandEncoder, RawGpuInstance, RawSurfaceTexture, RawWindowSurface};

use {plam::SurfaceSize, std::iter::once};

#[derive(Debug)]
pub struct RawGraphicsDevice {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl RawGraphicsDevice {
    pub(crate) async fn new(
        instance: &RawGpuInstance,
        surface: &RawWindowSurface,
    ) -> Result<Self, HellError> {
        let adapter = instance
            .raw_get()
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(surface.raw_get()),
                apply_limit_buckets: false,
            })
            .await
            .map_err(|_| HellError::AdapterRequest)?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Glumy Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::defaults(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|_| HellError::DeviceCreate)?;

        Ok(Self {
            adapter,
            device,
            queue,
        })
    }
}

impl RawGraphicsDevice {
    pub fn present(&self, encoder: RawCommandEncoder, frame: RawSurfaceTexture) {
        self.queue.submit(once(encoder.take().finish()));
        self.queue.present(frame.raw_take())
    }

    pub fn create_encoder(&self) -> RawCommandEncoder {
        RawCommandEncoder::from_wgpu(
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default()),
        )
    }
}

impl RawGraphicsDevice {
    pub(crate) fn raw_config(
        &self,
        surface: &RawWindowSurface,
        device: &RawGraphicsDevice,
        surface_size: SurfaceSize,
    ) -> wgpu::SurfaceConfiguration {
        let capabilities = surface.raw_get().get_capabilities(device.raw_adapter());

        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: surface_size.width(),
            height: surface_size.height(),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Srgb,
        };

        config
    }
}

impl RawGraphicsDevice {
    #[inline]
    pub(crate) fn raw_get(&self) -> &wgpu::Device {
        &self.device
    }

    #[inline]
    pub(crate) fn raw_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    #[inline]
    pub(crate) fn raw_adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }
}
