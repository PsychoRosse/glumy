use crate::{SurfaceTarget, WindowHandlerError};

pub trait WindowHandle {
    fn target(&self) -> Result<SurfaceTarget, WindowHandlerError>;
}
