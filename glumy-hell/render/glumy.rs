use crate::{
    GlumyDescriptor, GlumyHell, GlumyHellError, GpuInstance, GraphicsDevice, WindowSurface,
};

pub struct Glumy;

impl Glumy {
    pub fn create(desc: GlumyDescriptor) -> Result<GlumyHell, GlumyHellError> {
        let instance = GpuInstance::default();
        let mut surface = WindowSurface::new(&instance, desc.surface_target)?;
        let device = GraphicsDevice::new(&instance, &surface)?;

        let capabilities = surface.get().get_capabilities(device.adapter());

        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: desc.surface_size.width(),
            height: desc.surface_size.height(),
            present_mode: wgpu::PresentMode::from(desc.vsync),
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: desc.frame_latency,
            color_space: wgpu::SurfaceColorSpace::Srgb,
        };

        surface.set_config(config);
        surface.configure(&device);

        Ok(GlumyHell {
            instance,
            surface,
            device,
        })
    }
}
