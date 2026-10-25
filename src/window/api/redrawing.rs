use clama::Event;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Event, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Redrawing;
