use crate::{LoadOp, RawRenderPassDepthAttachment, StoreOp, TextureView};

pub struct RenderPassDepthAttachment<'r> {
    pub(crate) attach: RawRenderPassDepthAttachment<'r>,
}

impl<'r> RenderPassDepthAttachment<'r> {
    pub fn clear(view: &'r TextureView, depth: f32) -> Self {
        Self {
            attach: RawRenderPassDepthAttachment::new(
                view.raw_get(),
                LoadOp::Clear(depth),
                StoreOp::Store,
                None,
            ),
        }
    }

    pub fn load(view: &'r TextureView) -> Self {
        Self {
            attach: RawRenderPassDepthAttachment::new(
                view.raw_get(),
                LoadOp::Load,
                StoreOp::Store,
                None,
            ),
        }
    }

    pub fn with_stencil(mut self) -> Self {
        self.attach = self.attach.with_stencil();
        self
    }
}
