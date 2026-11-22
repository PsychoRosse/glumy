#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceTarget {
    raw_display: raw_window_handle::RawDisplayHandle,
    raw_window: raw_window_handle::RawWindowHandle,
}

impl SurfaceTarget {
    #[must_use]
    pub const fn new(
        raw_display: raw_window_handle::RawDisplayHandle,
        raw_window: raw_window_handle::RawWindowHandle,
    ) -> Self {
        Self {
            raw_display,
            raw_window,
        }
    }
}

impl SurfaceTarget {
    #[must_use]
    pub const fn raw_display(&self) -> raw_window_handle::RawDisplayHandle {
        self.raw_display
    }

    #[must_use]
    pub const fn raw_window(&self) -> raw_window_handle::RawWindowHandle {
        self.raw_window
    }
}
