use crate::{SurfaceTarget, WindowFrontend, WindowHandle, WindowHandlerError};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use winit::window;

pub struct WindowBackend {
    window: std::sync::Arc<dyn window::Window>,
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
    fn size(&self) -> plam::Extent {
        let size = self.window.surface_size();

        plam::Extent::clamped(size.width, size.height)
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

impl WindowBackend {
    pub fn from_box_winit_to_arc(window: Box<dyn window::Window>) -> Self {
        Self {
            window: std::sync::Arc::from(window),
        }
    }
}
