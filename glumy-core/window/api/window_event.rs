use crate::{KeyboardEvent, MouseEvent, MouseWheelEvent};

use plam::{MousePosition, SurfaceSize};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum WindowEvent {
    CloseRequested,
    RedrawRequested,
    SurfaceResized(SurfaceSize),
    KeyboardInput(KeyboardEvent),
    MouseInput(MouseEvent),
    PointerMoved(MousePosition),
    MouseWheel(MouseWheelEvent),
    Unknown,
}
