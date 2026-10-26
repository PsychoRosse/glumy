use crate::{
    GpuInstance, GraphicsDevice, InstanceError, SurfaceTarget, SurfaceTexture, TextureView,
    WhatFrame,
};

use plam::SurfaceSize;

#[cfg(feature = "clama")]
use clama::Resource;

#[cfg_attr(feature = "clama", derive(Resource))]
pub struct WindowSurface {
    surface: wgpu::Surface<'static>,
    config: Option<wgpu::SurfaceConfiguration>,
}

impl WindowSurface {
    pub fn new(instance: &GpuInstance, surface_target: SurfaceTarget) -> Self {
        let surface = unsafe {
            instance
                .get()
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: Some(surface_target.raw_display()),
                    raw_window_handle: surface_target.raw_window(),
                })
                .unwrap_or_else(|_| panic!("{}", InstanceError::SurfaceCreate))
        };

        Self {
            surface,
            config: None,
        }
    }
}

impl WindowSurface {
    pub fn acquire_view(&self) -> (Option<SurfaceTexture>, WhatFrame) {
        match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => {
                (Some(SurfaceTexture::new(frame)), WhatFrame::Success)
            }
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => {
                (Some(SurfaceTexture::new(frame)), WhatFrame::Suboptimal)
            }
            _ => (None, WhatFrame::Unknown),
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
    pub(crate) fn set_config(&mut self, config: Option<wgpu::SurfaceConfiguration>) {
        self.config = config
    }
    pub(crate) fn get<'s>(&self) -> &wgpu::Surface<'s> {
        &self.surface
    }

    pub(crate) fn config(&self) -> Option<&wgpu::SurfaceConfiguration> {
        self.config.as_ref()
    }
}
