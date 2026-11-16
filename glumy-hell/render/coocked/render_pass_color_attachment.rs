use crate::{Color, LoadOp, RawRenderPassColorAttachment, StoreOp, TextureView};

pub struct RenderPassColorAttachment<'r> {
    pub(crate) attach: RawRenderPassColorAttachment<'r>,
}

impl<'r> RenderPassColorAttachment<'r> {
    pub fn new(
        view: &'r TextureView,
        resolve_target: Option<&'r TextureView>,
        load_op: LoadOp<Color>,
        store_op: StoreOp,
    ) -> Self {
        Self {
            attach: RawRenderPassColorAttachment::new(
                view.raw_get(),
                resolve_target.map(TextureView::raw_get),
                load_op,
                store_op,
            ),
        }
    }
}

impl<'r> RenderPassColorAttachment<'r> {
    pub(crate) fn raw_get(&self) -> &RawRenderPassColorAttachment<'_> {
        &self.attach
    }
}
