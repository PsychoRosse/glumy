use crate::TextureView;

#[derive(Debug)]
pub struct RenderPassDepthAttach<'r> {
    pub view: &'r TextureView,

    pub(crate) depth_load_op: wgpu::LoadOp<f32>,
    pub(crate) depth_store_op: wgpu::StoreOp,

    pub(crate) stencil_load_op: Option<wgpu::LoadOp<u32>>,
    pub(crate) stencil_store_op: Option<wgpu::StoreOp>,
}

impl<'r> RenderPassDepthAttach<'r> {
    pub fn clear(view: &'r TextureView, depth: f32) -> Self {
        Self {
            view,
            depth_load_op: wgpu::LoadOp::Clear(depth),
            depth_store_op: wgpu::StoreOp::Store,
            stencil_load_op: None,
            stencil_store_op: None,
        }
    }

    pub fn load(view: &'r TextureView) -> Self {
        Self {
            view,
            depth_load_op: wgpu::LoadOp::Load,
            depth_store_op: wgpu::StoreOp::Store,
            stencil_load_op: None,
            stencil_store_op: None,
        }
    }
}
