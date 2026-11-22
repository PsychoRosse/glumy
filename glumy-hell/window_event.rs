use crate::{KeyboardEvent, MouseEvent, MouseWheelEvent};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowEvent {
    CloseRequested,
    RedrawRequested,
    SurfaceResized(plam::SurfaceSize),
    KeyboardInput(KeyboardEvent),
    MouseInput(MouseEvent),
    PointerMoved(plam::MousePosition),
    MouseWheel(MouseWheelEvent),
}
