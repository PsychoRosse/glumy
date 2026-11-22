use crate::{Closing, KeyboardEvent, MouseEvent, MouseWheelEvent, Redrawing};

pub struct WindowEventPlugin;

impl clama::Plugin for WindowEventPlugin {
    fn build(&self, registry: &mut impl clama::Registry) {
        registry
            .with_event(clama::EventBuffer::<Closing>::default())
            .with_event(clama::EventBuffer::<Redrawing>::default())
            .with_event(clama::EventBuffer::<plam::SurfaceSize>::default());

        registry
            .with_event(clama::EventBuffer::<KeyboardEvent>::default())
            .with_event(clama::EventBuffer::<MouseEvent>::default())
            .with_event(clama::EventBuffer::<plam::MousePosition>::default())
            .with_event(clama::EventBuffer::<MouseWheelEvent>::default());
    }
}
