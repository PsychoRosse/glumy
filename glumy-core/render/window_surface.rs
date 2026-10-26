use crate::{GraphicsDevice, InstanceError, TextureView, WindowSurfaceDescriptor};

use plam::SurfaceSize;

pub struct WindowSurface {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
}

impl WindowSurface {
    pub fn new(desc: &WindowSurfaceDescriptor) -> Self {
        let surface = unsafe {
            desc.instance
                .get()
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: Some(desc.surface_target.raw_display()),
                    raw_window_handle: desc.surface_target.raw_window(),
                })
                .unwrap_or_else(|_| panic!("{}", InstanceError::SurfaceCreate))
        };

        let capabilities = surface.get_capabilities(desc.device.adapter());

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
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Srgb,
        };

        surface.configure(desc.device.get(), &config);

        Self { surface, config }
    }
}

impl WindowSurface {
    pub fn acquire_view(&self) -> Option<TextureView> {
        match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => Some(Self::create_view(&frame)),
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => Some(Self::create_view(&frame)),
            _ => None,
        }
    }
}

impl WindowSurface {
    pub(crate) fn get<'s>(&self) -> &wgpu::Surface<'s> {
        &self.surface
    }

    pub(crate) fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub(crate) fn resize(&mut self, device: &GraphicsDevice, size: SurfaceSize) {
        self.config.width = size.width();
        self.config.height = size.height();

        self.surface.configure(device.get(), &self.config);
    }

    pub(crate) fn create_view(frame: &wgpu::SurfaceTexture) -> TextureView {
        TextureView::new(
            frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        )
    }
}
