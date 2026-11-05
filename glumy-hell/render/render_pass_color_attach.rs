use crate::{Color, TextureView};

#[derive(Debug)]
pub struct RenderPassColorAttach<'r> {
    pub view: &'r TextureView,
    pub resolve_target: Option<&'r TextureView>,

    pub(crate) load_op: wgpu::LoadOp<wgpu::Color>,
    pub(crate) store_op: wgpu::StoreOp,
}

impl<'r> RenderPassColorAttach<'r> {
    pub fn clear(view: &'r TextureView, color: Color) -> Self {
        Self {
            view,
            resolve_target: None,
            load_op: wgpu::LoadOp::Clear(wgpu::Color::from(color)),
            store_op: wgpu::StoreOp::Store,
        }
    }

    pub fn load(view: &'r TextureView) -> Self {
        Self {
            view,
            resolve_target: None,
            load_op: wgpu::LoadOp::Load,
            store_op: wgpu::StoreOp::Store,
        }
    }
}
