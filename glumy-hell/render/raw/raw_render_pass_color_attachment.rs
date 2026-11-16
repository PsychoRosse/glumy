use crate::{Color, LoadOp, RawTextureView, StoreOp};

#[derive(Debug, Clone, Copy)]
pub struct RawRenderPassColorAttachment<'r> {
    pub(crate) view: &'r wgpu::TextureView,
    pub(crate) resolve_target: Option<&'r wgpu::TextureView>,
    pub(crate) load_op: wgpu::LoadOp<wgpu::Color>,
    pub(crate) store_op: wgpu::StoreOp,
}

impl<'r> RawRenderPassColorAttachment<'r> {
    pub fn new(
        view: &'r RawTextureView,
        resolve_target: Option<&'r RawTextureView>,
        load_op: LoadOp<Color>,
        store_op: StoreOp,
    ) -> Self {
        Self {
            view: view.raw_get(),
            resolve_target: resolve_target.map(RawTextureView::raw_get),
            load_op: load_op.to_wgpu_color(),
            store_op: store_op.to_wgpu(),
        }
    }
}
