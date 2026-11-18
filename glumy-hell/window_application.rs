use crate::{
    Application, EventLoopError, WindowAttributes, WindowBackend, WindowError, WindowEvent,
    WindowFrontend, WindowId, WinitEventConverter,
};

use winit::{application, event, event_loop, window};

pub struct WindowApplication<A: Application> {
    application: A,
    window: Option<WindowBackend>,
    attributes: WindowAttributes,
}

impl<A: Application> std::fmt::Debug for WindowApplication<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("WindowApplication")
            .field("Application", &"<dyn Application>")
            .field("Attributes", &self.attributes)
            .finish()
    }
}

impl<A: Application> WindowApplication<A> {
    pub fn new(attributes: WindowAttributes, application: A) -> Self {
        Self {
            application,
            window: None,
            attributes,
        }
    }
}

impl<A: Application> WindowApplication<A> {
    pub fn run(self) -> Result<(), EventLoopError> {
        let event_loop =
            event_loop::EventLoop::new().map_err(|_| EventLoopError::EventLoopCreation)?;

        event_loop
            .run_app(self)
            .map_err(|_| EventLoopError::EventLoopRun)?;

        Ok(())
    }
}

impl<A: Application> application::ApplicationHandler for WindowApplication<A> {
    #[inline(always)]
    fn window_event(
        &mut self,
        event_loop: &dyn event_loop::ActiveEventLoop,
        window_id: window::WindowId,
        window_event: event::WindowEvent,
    ) {
        let Some(window) = &self.window else { return };

        let Some(event) = WinitEventConverter::from(window_event) else {
            return;
        };

        self.application
            .event(event, WindowId::new(window_id.into_raw()));

        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }

        window.redraw();
    }

    #[inline(always)]
    fn about_to_wait(&mut self, _: &dyn event_loop::ActiveEventLoop) {
        self.application.tick()
    }

    fn can_create_surfaces(&mut self, event_loop: &dyn event_loop::ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = event_loop
            .create_window(window::WindowAttributes::from(self.attributes))
            .unwrap_or_else(|_| panic!("{}", WindowError::WindowCreation));

        let window = WindowBackend::from(window);

        self.application.start(&window, &window);

        self.window = Some(window)
    }

    fn resumed(&mut self, _: &dyn event_loop::ActiveEventLoop) {}
}
