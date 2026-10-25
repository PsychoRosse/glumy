use crate::{Closing, KeyboardEvent, MouseEvent, MouseWheelEvent, Redrawing};

pub use {
    clama::{EventBuffer, Plugin, Registry},
    plam::{MousePosition, SurfaceSize},
};

#[derive(Clone, Copy)]
pub struct WindowEventPlugin;

impl Plugin for WindowEventPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_event(EventBuffer::<Closing>::default())
            .with_event(EventBuffer::<Redrawing>::default())
            .with_event(EventBuffer::<SurfaceSize>::default());

        registry
            .with_event(EventBuffer::<KeyboardEvent>::default())
            .with_event(EventBuffer::<MouseEvent>::default())
            .with_event(EventBuffer::<MousePosition>::default())
            .with_event(EventBuffer::<MouseWheelEvent>::default());
    }
}
