use crate::{RenderPassColorAttachment, RenderPassDepthAttachment};

pub struct RenderPassDescriptor<'r> {
    pub label: String,
    pub color_attaches: &'r [RenderPassColorAttachment<'r>],
    pub depth_attach: Option<&'r RenderPassDepthAttachment<'r>>,
}
