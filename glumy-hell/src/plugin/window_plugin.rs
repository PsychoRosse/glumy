use crate::{InputPlugin, TimePlugin, WindowEventPlugin};

pub use clama::{Plugin, Registry};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_plugin(TimePlugin)
            .with_plugin(WindowEventPlugin)
            .with_plugin(InputPlugin);
    }
}
