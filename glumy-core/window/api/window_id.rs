#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WindowId(usize);

impl WindowId {
    #[inline]
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl WindowId {
    #[inline]
    #[must_use]
    pub fn get(&self) -> usize {
        self.0
    }
}
