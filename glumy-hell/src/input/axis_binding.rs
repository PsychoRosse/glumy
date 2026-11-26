use crate::Key;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AxisBinding {
    positive: Key,
    negative: Key,
}

impl AxisBinding {
    #[must_use]
    pub fn new(positive: Key, negative: Key) -> Self {
        Self { positive, negative }
    }
}

impl AxisBinding {
    #[inline]
    #[must_use]
    pub fn positive(&self) -> Key {
        self.positive
    }

    #[inline]
    #[must_use]
    pub fn negative(&self) -> Key {
        self.negative
    }
}
