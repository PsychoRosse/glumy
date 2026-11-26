#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(clama::Event, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Closing;
