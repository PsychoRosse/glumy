use crate::{RawRenderPassDepthAttachment, TextureView};

pub struct RenderPassDepthAttachment<'r> {
    pub(crate) attach: RawRenderPassDepthAttachment<'r>,
}

impl<'r> RenderPassDepthAttachment<'r> {
    pub fn clear(view: &'r TextureView, depth: f32) -> Self {
        Self {
            attach: RawRenderPassDepthAttachment::clear(view.raw_get(), depth),
        }
    }

    pub fn load(view: &'r TextureView) -> Self {
        Self {
            attach: RawRenderPassDepthAttachment::load(view.raw_get()),
        }
    }

    pub fn with_stencil(mut self) -> Self {
        self.attach = self.attach.with_stencil();
        self
    }
}
