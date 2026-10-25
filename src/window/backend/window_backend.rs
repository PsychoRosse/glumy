use crate::{SurfaceTarget, WindowFrontend, WindowHandle, WindowHandlerError};

use {
    plam::SurfaceSize,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    std::sync::Arc,
};

use winit::window;

pub struct WindowBackend {
    window: Arc<dyn window::Window>,
}

impl std::fmt::Debug for WindowBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("WinitWindowBackend")
            .field("Window", &"dyn Window")
            .field("Id", &self.window.id())
            .field("Title", &self.window.title())
            .finish()
    }
}

impl WindowFrontend for WindowBackend {
    #[inline]
    fn redraw(&self) {
        self.window.request_redraw();
    }

    #[inline]
    fn size(&self) -> SurfaceSize {
        let size = self.window.surface_size();

        SurfaceSize::clamped(size.width, size.height)
    }
}

impl WindowHandle for WindowBackend {
    fn target(&self) -> Result<SurfaceTarget, WindowHandlerError> {
        let window = self
            .window
            .display_handle()
            .map_err(|_| WindowHandlerError::RawDisplayHandle)?
            .as_raw();

        let display = self
            .window
            .window_handle()
            .map_err(|_| WindowHandlerError::RawWindowHandle)?
            .as_raw();

        Ok(SurfaceTarget::new(window, display))
    }
}

impl From<Box<dyn window::Window>> for WindowBackend {
    fn from(value: Box<dyn window::Window>) -> Self {
        Self {
            window: std::sync::Arc::from(value),
        }
    }
}
