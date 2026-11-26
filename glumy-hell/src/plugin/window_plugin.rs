use crate::{InputPlugin, TimePlugin};

pub use clama::{Plugin, Registry};

pub struct WindowEventPlugin;

impl Plugin for WindowEventPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_plugin(WindowEventPlugin)
            .with_plugin(TimePlugin)
            .with_plugin(InputPlugin);
    }
}
