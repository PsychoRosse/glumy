use glumy::{Application, EventLoopError, WindowApplication, WindowAttributes};

fn main() -> Result<(), EventLoopError> {
    GlumyApp::default().run_from(WindowAttributes::default())
}

#[derive(Default)]
struct GlumyApp;

impl GlumyApp {
    pub fn run_from(self, attributes: WindowAttributes) -> Result<(), EventLoopError> {
        WindowApplication::new(attributes, self).run()
    }
}

impl Application for GlumyApp {}
