use plam::MouseScroll;

use clama::Event;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Event, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseWheelEvent {
    scroll: MouseScroll,
}

impl From<MouseScroll> for MouseWheelEvent {
    #[inline]
    fn from(value: MouseScroll) -> Self {
        Self { scroll: value }
    }
}

impl MouseWheelEvent {
    #[inline]
    #[must_use]
    pub fn scroll(&self) -> MouseScroll {
        self.scroll
    }
}
