use crate::{RawRenderPassColorAttachment, TextureView};

pub struct RenderPassColorAttachment<'r> {
    pub(crate) attach: RawRenderPassColorAttachment<'r>,
}

impl<'r> RenderPassColorAttachment<'r> {
    pub fn new(view: &'r TextureView, resolve_target: Option<&'r TextureView>) -> Self {
        Self {
            attach: RawRenderPassColorAttachment::new(
                view.raw_get(),
                resolve_target.map(TextureView::raw_get),
            ),
        }
    }
}

impl<'r> RenderPassColorAttachment<'r> {
    pub(crate) fn raw_get(&self) -> RawRenderPassColorAttachment<'_> {
        self.attach
    }
}
