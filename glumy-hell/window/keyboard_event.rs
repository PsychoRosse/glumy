use crate::{Key, KeyStatus};

use clama::Event;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardEvent {
    key: Key,
    state: KeyStatus,
}

impl KeyboardEvent {
    #[inline]
    #[must_use]
    pub const fn new(key: Key, state: KeyStatus) -> Self {
        Self { key, state }
    }
}

impl KeyboardEvent {
    #[inline]
    #[must_use]
    pub const fn key(&self) -> Key {
        self.key
    }

    #[inline]
    #[must_use]
    pub const fn state(&self) -> KeyStatus {
        self.state
    }
}
