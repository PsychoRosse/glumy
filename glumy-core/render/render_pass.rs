use crate::{CommandEncoder, TextureView};

pub struct RenderPass<'r> {
    _pass: wgpu::RenderPass<'r>,
}

impl<'a> RenderPass<'a> {
    pub fn begin(encoder: &'a mut CommandEncoder, color: &TextureView) -> Self {
        Self {
            _pass: encoder
                .get_mut()
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Glumy Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: color.get(),
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                }),
        }
    }
}
