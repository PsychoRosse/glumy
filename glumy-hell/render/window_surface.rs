use crate::{
    AcquireFrame, GlumyHellError, GpuInstance, GraphicsDevice, SurfaceTarget, SurfaceTexture,
    WhatFrame,
};

use {clama::Resource, plam::SurfaceSize};

#[derive(Resource, Debug)]
pub struct WindowSurface {
    surface: wgpu::Surface<'static>,
    config: Option<wgpu::SurfaceConfiguration>,
}

impl WindowSurface {
    pub fn new(
        instance: &GpuInstance,
        surface_target: SurfaceTarget,
    ) -> Result<Self, GlumyHellError> {
        let surface = unsafe {
            instance
                .get()
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: Some(surface_target.raw_display()),
                    raw_window_handle: surface_target.raw_window(),
                })
                .map_err(|_| GlumyHellError::SurfaceCreate)?
        };

        Ok(Self {
            surface,
            config: None,
        })
    }
}

impl WindowSurface {
    pub fn acquire_frame(&self) -> AcquireFrame {
        match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => {
                AcquireFrame::Frame(Some(SurfaceTexture::new(frame)), WhatFrame::Success)
            }
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => {
                AcquireFrame::Frame(Some(SurfaceTexture::new(frame)), WhatFrame::Suboptimal)
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                AcquireFrame::Frame(None, WhatFrame::Validation)
            }
            wgpu::CurrentSurfaceTexture::Timeout => AcquireFrame::Frame(None, WhatFrame::Timeout),
            wgpu::CurrentSurfaceTexture::Outdated => AcquireFrame::Frame(None, WhatFrame::Outdated),
            wgpu::CurrentSurfaceTexture::Occluded => AcquireFrame::Frame(None, WhatFrame::Occluded),
            wgpu::CurrentSurfaceTexture::Lost => AcquireFrame::Frame(None, WhatFrame::Lost),
        }
    }

    pub fn configure(&self, device: &GraphicsDevice) {
        if let Some(config) = &self.config {
            self.surface.configure(device.get(), config);
        }
    }

    pub fn resize(&mut self, device: &GraphicsDevice, size: SurfaceSize) {
        if let Some(config) = &mut self.config {
            config.width = size.width();
            config.height = size.height();

            self.surface.configure(device.get(), config);
        }
    }
}

impl WindowSurface {
    pub(crate) fn set_config(&mut self, config: wgpu::SurfaceConfiguration) {
        self.config = Some(config)
    }

    #[allow(dead_code)]
    pub(crate) fn format(&self) -> Option<wgpu::TextureFormat> {
        if let Some(config) = &self.config {
            return Some(config.format);
        }

        None
    }

    #[allow(dead_code)]
    pub(crate) fn config(&self) -> Option<&wgpu::SurfaceConfiguration> {
        self.config.as_ref()
    }

    pub(crate) fn get<'s>(&self) -> &wgpu::Surface<'s> {
        &self.surface
    }
}
