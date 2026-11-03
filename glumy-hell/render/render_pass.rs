use crate::{CommandEncoder, TextureView};

#[derive(Debug)]
pub struct RenderPass<'r> {
    _pass: wgpu::RenderPass<'r>,
}

impl<'r> RenderPass<'r> {
    pub fn begin(encoder: &'r mut CommandEncoder, color: &TextureView) -> Self {
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
                                r: 0.05,
                                g: 0.05,
                                b: 0.05,
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
