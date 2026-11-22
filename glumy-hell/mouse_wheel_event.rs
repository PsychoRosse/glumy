#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(clama::Event, Debug, Clone, Copy, PartialEq)]
pub struct MouseWheelEvent {
    scroll: plam::MouseScroll,
}

impl MouseWheelEvent {
    #[inline]
    #[must_use]
    pub fn from_mouse_scroll(scroll: plam::MouseScroll) -> Self {
        Self { scroll: scroll }
    }

    #[inline]
    #[must_use]
    pub const fn scroll(&self) -> plam::MouseScroll {
        self.scroll
    }
}
