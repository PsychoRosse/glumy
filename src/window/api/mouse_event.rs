use crate::{Button, ButtonStatus};

#[cfg(feature = "clama")]
use clama::Event;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "clama", derive(Event))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MouseEvent {
    button: Button,
    state: ButtonStatus,
}

impl MouseEvent {
    #[inline]
    pub fn new(button: Button, state: ButtonStatus) -> Self {
        Self { button, state }
    }
}

impl MouseEvent {
    #[inline]
    #[must_use]
    pub fn button(&self) -> Button {
        self.button
    }

    #[inline]
    #[must_use]
    pub fn state(&self) -> ButtonStatus {
        self.state
    }
}
