use crate::{WindowEvent, WindowFrontend, WindowHandle, WindowId};

pub trait Application: 'static {
    fn start(&mut self, _window: &impl WindowFrontend, _handle: &impl WindowHandle) {}

    fn event(&mut self, event: WindowEvent, id: WindowId);

    fn tick(&mut self) {}
}
