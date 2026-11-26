use crate::{WindowEvent, WindowFrontend, WindowHandle, WindowId};

#[allow(unused_variables)]
pub trait Application: 'static {
    fn start(&mut self, window: &impl WindowFrontend, handle: &impl WindowHandle) {}

    fn event(&mut self, event: WindowEvent, id: WindowId) {}

    fn tick(&mut self) {}
}
