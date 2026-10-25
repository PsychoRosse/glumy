use plam::MouseScroll;

#[cfg(feature = "clama")]
use clama::Event;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "clama", derive(Event))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
