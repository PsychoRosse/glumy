use crate::{RenderPassColorAttach, RenderPassDepthAttach};

pub struct RenderPassDescriptor<'r> {
    pub color_attaches: &'r [RenderPassColorAttach<'r>],
    pub depth_attach: Option<&'r RenderPassDepthAttach<'r>>,
}
