use crate::{RawCommandEncoder, RawRenderPassEncoder, RenderPassDescriptor};

use smallvec::SmallVec;

pub struct RawRenderPass;

impl RawRenderPass {
    pub fn begin<R>(
        encoder: &mut RawCommandEncoder,
        desc: &RenderPassDescriptor<'_>,
        f: impl FnOnce(&mut RawRenderPassEncoder<'_>) -> R,
    ) -> R {
        let color_attachments: SmallVec<[Option<wgpu::RenderPassColorAttachment<'_>>; 4]> = desc
            .color_attaches
            .iter()
            .map(|attachment| {
                let attachment = &attachment.attach;

                Some(wgpu::RenderPassColorAttachment {
                    view: attachment.view,
                    depth_slice: None,
                    resolve_target: attachment.resolve_target,
                    ops: wgpu::Operations {
                        load: attachment.load_op,
                        store: attachment.store_op,
                    },
                })
            })
            .collect();

        let depth_attachment = desc.depth_attach.map(|attachment| {
            let attachment = &attachment.attach;

            wgpu::RenderPassDepthStencilAttachment {
                view: attachment.view,
                depth_ops: Some(wgpu::Operations {
                    load: attachment.depth_load_op,
                    store: attachment.depth_store_op,
                }),
                stencil_ops: attachment
                    .stencil_ops
                    .map(|(load, store)| wgpu::Operations { load, store }),
            }
        });

        let raw = encoder
            .raw_get_mut()
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(&desc.label),
                color_attachments: &color_attachments,
                depth_stencil_attachment: depth_attachment,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

        let mut pass = RawRenderPassEncoder::from_wgpu(raw);

        f(&mut pass)
    }
}
