use glumy::{EventLoopError, GlumyApp, RenderPlugin, WindowAttributes};

fn main() -> Result<(), EventLoopError> {
    GlumyApp::default()
        .with_plugin(RenderPlugin)
        .run_from(WindowAttributes::default())
}
