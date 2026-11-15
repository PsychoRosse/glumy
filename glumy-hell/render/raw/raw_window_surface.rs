use crate::{
    CurrentSurfaceTextureError, HellError, RawCurrentSurfaceTexture, RawGpuInstance,
    RawGraphicsDevice, SurfaceTarget, TextureFormat,
};

use plam::SurfaceSize;

#[derive(Debug)]
pub(crate) struct RawWindowSurface {
    surface: wgpu::Surface<'static>,
    config: Option<wgpu::SurfaceConfiguration>,
}

impl RawWindowSurface {
    pub fn new(
        instance: &RawGpuInstance,
        surface_target: SurfaceTarget,
    ) -> Result<Self, HellError> {
        let surface = unsafe {
            instance
                .raw_get()
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: Some(surface_target.raw_display()),
                    raw_window_handle: surface_target.raw_window(),
                })
                .map_err(|_| HellError::SurfaceCreate)?
        };

        Ok(Self {
            surface,
            config: None,
        })
    }
}

impl RawWindowSurface {
    pub fn format(&self) -> Result<TextureFormat, CurrentSurfaceTextureError> {
        if let Some(config) = &self.config {
            return TextureFormat::from_wgpu(config.format);
        }

        Err(CurrentSurfaceTextureError::SurfaceTextureNotCreated)
    }

    pub fn size(&self) -> Option<SurfaceSize> {
        if let Some(config) = &self.config {
            return Some(SurfaceSize::clamped(config.width, config.height));
        }

        None
    }

    pub fn resize(&mut self, device: &RawGraphicsDevice, size: SurfaceSize) -> bool {
        if let Some(config) = &mut self.config {
            config.width = size.width();
            config.height = size.height();

            self.surface.configure(device.raw_get(), &config);

            return true;
        }

        return false;
    }
}

impl RawWindowSurface {
    pub fn configure(&self, device: &RawGraphicsDevice) {
        if let Some(config) = &self.config {
            self.surface.configure(device.raw_get(), config);
        }
    }
}

impl RawWindowSurface {
    pub fn acquire_frame(&self) -> RawCurrentSurfaceTexture {
        RawCurrentSurfaceTexture::from_wgpu(self.surface.get_current_texture())
    }
}

impl RawWindowSurface {
    pub(crate) fn set_config(&mut self, config: wgpu::SurfaceConfiguration) {
        self.config = Some(config)
    }
}

impl RawWindowSurface {
    pub(crate) fn raw_get<'s>(&self) -> &wgpu::Surface<'s> {
        &self.surface
    }

    pub(crate) fn raw_config(&self) -> Option<&wgpu::SurfaceConfiguration> {
        self.config.as_ref()
    }
}
