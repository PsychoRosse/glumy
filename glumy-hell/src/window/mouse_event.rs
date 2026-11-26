use crate::{Button, ButtonStatus};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(clama::Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseEvent {
    button: Button,
    state: ButtonStatus,
}

impl MouseEvent {
    #[inline]
    #[must_use]
    pub const fn new(button: Button, state: ButtonStatus) -> Self {
        Self { button, state }
    }
}

impl MouseEvent {
    #[inline]
    #[must_use]
    pub const fn button(&self) -> Button {
        self.button
    }

    #[inline]
    #[must_use]
    pub const fn state(&self) -> ButtonStatus {
        self.state
    }
}
