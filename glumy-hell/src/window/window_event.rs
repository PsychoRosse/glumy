use crate::{KeyboardEvent, MouseEvent, MouseWheelEvent};

use plam::{Extent, MousePosition};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowEvent {
    CloseRequested,
    RedrawRequested,
    SurfaceResized(Extent),
    KeyboardInput(KeyboardEvent),
    MouseInput(MouseEvent),
    PointerMoved(MousePosition),
    MouseWheel(MouseWheelEvent),
}
