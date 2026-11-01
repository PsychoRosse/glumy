use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WindowHandlerError {
    #[error("Failed to get window handle")]
    RawWindowHandle,
    #[error("Failed to get display handle")]
    RawDisplayHandle,
}
