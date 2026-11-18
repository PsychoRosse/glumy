#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId {
    id: usize,
}

impl WindowId {
    #[inline]
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self { id }
    }
}

impl WindowId {
    #[inline]
    #[must_use]
    pub const fn get(&self) -> usize {
        self.id
    }
}
