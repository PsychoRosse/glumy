use crate::{CommandEncoder, RawRenderPass, RenderPassDescriptor, RenderPassEncoder};

pub struct RenderPass;

impl RenderPass {
    pub fn begin<R>(
        encoder: &mut CommandEncoder,
        desc: &RenderPassDescriptor<'_>,
        f: impl FnOnce(&mut RenderPassEncoder<'_, '_>) -> R,
    ) -> R {
        RawRenderPass::begin(encoder.raw_get_mut(), desc, |raw| {
            let mut pass = RenderPassEncoder::from_raw(raw);
            f(&mut pass)
        })
    }
}
