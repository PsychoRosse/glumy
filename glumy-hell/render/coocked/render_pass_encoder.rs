use crate::RawRenderPassEncoder;

pub struct RenderPassEncoder<'a, 'p> {
    raw: &'a mut RawRenderPassEncoder<'p>,
}

impl<'a, 'p> RenderPassEncoder<'a, 'p> {
    pub(crate) fn from_raw(raw: &'a mut RawRenderPassEncoder<'p>) -> Self {
        Self { raw }
    }
}
