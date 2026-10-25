use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceTarget {
    raw_display: RawDisplayHandle,
    raw_window: RawWindowHandle,
}

impl SurfaceTarget {
    #[must_use]
    pub fn new(raw_display: RawDisplayHandle, raw_window: RawWindowHandle) -> Self {
        Self {
            raw_display,
            raw_window,
        }
    }
}

impl SurfaceTarget {
    #[inline]
    #[must_use]
    pub fn raw_display(&self) -> RawDisplayHandle {
        self.raw_display
    }

    #[inline]
    #[must_use]
    pub fn raw_window(&self) -> RawWindowHandle {
        self.raw_window
    }
}
