use crate::{CommandEncoder, RenderPassColorAttach, RenderPassDepthAttach};

use smallvec::SmallVec;

const INLINE_COLOR_ATTACHMENTS: usize = 4;

type ColorAttaches<'r> =
    Option<SmallVec<[Option<wgpu::RenderPassColorAttachment<'r>>; INLINE_COLOR_ATTACHMENTS]>>;

type DepthAttach<'r> = Option<wgpu::RenderPassDepthStencilAttachment<'r>>;

#[derive(Default, Debug)]
pub struct RenderPass<'r> {
    _pass: Option<wgpu::RenderPass<'r>>,
    color_attaches: ColorAttaches<'r>,
    depth_attach: DepthAttach<'r>,
}

impl<'r> RenderPass<'r> {
    pub fn with_colors(mut self, colors: &[RenderPassColorAttach<'r>]) -> Self {
        self.color_attaches = Some(
            colors
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
                .collect(),
        );

        self
    }

    pub fn with_depth(mut self, depth: Option<&RenderPassDepthAttach<'r>>) -> Self {
        self.depth_attach = depth.map(|attachment| wgpu::RenderPassDepthStencilAttachment {
            view: attachment.view.get(),

            depth_ops: Some(wgpu::Operations {
                load: attachment.depth_load_op,
                store: attachment.depth_store_op,
            }),

            stencil_ops: attachment
                .stencil_ops
                .map(|(load, store)| wgpu::Operations { load, store }),
        });

        self
    }

    pub fn begin(self, encoder: &'r mut CommandEncoder) {
        encoder
            .get_mut()
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Glumy Render Pass"),
                color_attachments: &self.color_attaches.unwrap_or_default(),
                depth_stencil_attachment: self.depth_attach,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
    }
}
