use crate::{CommandEncoder, RenderPassDescriptor};

use smallvec::SmallVec;

const INLINE_COLOR_ATTACHMENTS: usize = 4;

#[derive(Default, Debug)]
pub struct RenderPass<'r> {
    color_attaches:
        SmallVec<[Option<wgpu::RenderPassColorAttachment<'r>>; INLINE_COLOR_ATTACHMENTS]>,
    depth_attach: Option<wgpu::RenderPassDepthStencilAttachment<'r>>,
}
impl<'r> RenderPass<'r> {
    pub fn begin<R>(
        encoder: &mut CommandEncoder,
        desc: RenderPassDescriptor<'_>,
        f: impl FnOnce(&mut wgpu::RenderPass<'_>) -> R,
    ) -> R {
        let color_attaches: SmallVec<
            [Option<wgpu::RenderPassColorAttachment<'_>>; INLINE_COLOR_ATTACHMENTS],
        > = desc
            .color_attaches
            .iter()
            .map(|attachment| {
                Some(wgpu::RenderPassColorAttachment {
                    view: attachment.view.get(),
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: attachment.load_op,
                        store: attachment.store_op,
                    },
                })
            })
            .collect();

        let depth_attach =
            desc.depth_attach
                .map(|attachment| wgpu::RenderPassDepthStencilAttachment {
                    view: attachment.view.get(),
                    depth_ops: Some(wgpu::Operations {
                        load: attachment.depth_load_op,
                        store: attachment.depth_store_op,
                    }),
                    stencil_ops: attachment
                        .stencil_ops
                        .map(|(load, store)| wgpu::Operations { load, store }),
                });

        let mut pass = encoder
            .get_mut()
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Glumy Render Pass"),
                color_attachments: &color_attaches,
                depth_stencil_attachment: depth_attach,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

        f(&mut pass)
    }
}
