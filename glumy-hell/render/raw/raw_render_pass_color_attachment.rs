use crate::RawTextureView;

#[derive(Debug, Clone, Copy)]
pub struct RawRenderPassColorAttachment<'r> {
    pub(crate) view: &'r wgpu::TextureView,
    pub(crate) resolve_target: Option<&'r wgpu::TextureView>,
    pub(crate) load_op: wgpu::LoadOp<wgpu::Color>,
    pub(crate) store_op: wgpu::StoreOp,
}

impl<'r> RawRenderPassColorAttachment<'r> {
    pub fn new(view: &'r RawTextureView, resolve_target: Option<&'r RawTextureView>) -> Self {
        Self {
            view: view.raw_get(),
            resolve_target: resolve_target.map(RawTextureView::raw_get),
            load_op: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            store_op: wgpu::StoreOp::Store,
        }
    }
}
