use crate::TextureView;

#[derive(Debug)]
pub struct RenderPassDepthAttach<'r> {
    pub view: &'r TextureView,

    pub(crate) depth_load_op: wgpu::LoadOp<f32>,
    pub(crate) depth_store_op: wgpu::StoreOp,
    pub(crate) stencil_ops: Option<(wgpu::LoadOp<u32>, wgpu::StoreOp)>,
}

impl<'r> RenderPassDepthAttach<'r> {
    pub fn clear(view: &'r TextureView, depth: f32) -> Self {
        Self {
            view,
            depth_load_op: wgpu::LoadOp::Clear(depth),
            depth_store_op: wgpu::StoreOp::Store,
            stencil_ops: None,
        }
    }

    pub fn load(view: &'r TextureView) -> Self {
        Self {
            view,
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
