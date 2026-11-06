use glumy::{App, EventLoopError, RenderPlugin, WindowAttributes};

fn main() -> Result<(), EventLoopError> {
    App::default()
        .with_plugin(RenderPlugin)
        .run_from(WindowAttributes::default())
}
