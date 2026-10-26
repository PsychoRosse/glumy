use crate::{Adapter, GpuInstance, GraphicsDevice, SurfaceTarget, WindowSurface};

use plam::SurfaceSize;

pub struct Glumy;

impl Glumy {
    pub fn create(
        surface_target: SurfaceTarget,
        surface_size: SurfaceSize,
    ) -> (GpuInstance, WindowSurface, Adapter, GraphicsDevice) {
        let instance = GpuInstance::default();
        let mut surface = WindowSurface::new(&instance, surface_target);
        let adapter = Adapter::new(&instance, &surface);

        let capabilities = surface.get().get_capabilities(adapter.get());

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

        surface.set_config(Some(config));

        let device = GraphicsDevice::new(&adapter);

        (instance, surface, adapter, device)
    }
}
