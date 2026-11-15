use crate::RawTextureView;

pub struct RawRenderPassDepthAttachment<'r> {
    pub(crate) view: &'r wgpu::TextureView,
    pub(crate) depth_load_op: wgpu::LoadOp<f32>,
    pub(crate) depth_store_op: wgpu::StoreOp,
    pub(crate) stencil_ops: Option<(wgpu::LoadOp<u32>, wgpu::StoreOp)>,
}

impl<'r> RawRenderPassDepthAttachment<'r> {
    pub fn clear(view: &'r RawTextureView, depth: f32) -> Self {
        Self {
            view: view.raw_get(),
            depth_load_op: wgpu::LoadOp::Clear(depth),
            depth_store_op: wgpu::StoreOp::Store,
            stencil_ops: None,
        }
    }

    pub fn load(view: &'r RawTextureView) -> Self {
        Self {
            view: view.raw_get(),
            depth_load_op: wgpu::LoadOp::Load,
            depth_store_op: wgpu::StoreOp::Store,
            stencil_ops: None,
        }
    }

    pub fn with_stencil(mut self) -> Self {
        self.stencil_ops = Some((wgpu::LoadOp::Clear(0), wgpu::StoreOp::Store));
        self
    }
}
