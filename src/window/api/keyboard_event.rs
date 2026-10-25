use crate::{Key, KeyStatus};

use clama::Event;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeyboardEvent {
    key: Key,
    state: KeyStatus,
}

impl KeyboardEvent {
    #[inline]
    #[must_use]
    pub fn new(key: Key, state: KeyStatus) -> Self {
        Self { key, state }
    }
}

impl KeyboardEvent {
    #[inline]
    #[must_use]
    pub fn key(&self) -> Key {
        self.key
    }

    #[inline]
    #[must_use]
    pub fn state(&self) -> KeyStatus {
        self.state
    }
}
