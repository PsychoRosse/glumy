use crate::{CommandEncoder, RenderPassColorAttach, RenderPassDepthAttach};

#[derive(Default, Debug)]
pub struct RenderPass<'r> {
    _pass: Option<wgpu::RenderPass<'r>>,
    color_attaches: Vec<Option<wgpu::RenderPassColorAttachment<'r>>>,
    depth_attach: Option<wgpu::RenderPassDepthStencilAttachment<'r>>,
}

impl<'r> RenderPass<'r> {
    pub fn with_colors(mut self, colors: &[RenderPassColorAttach<'r>]) -> Self {
        let attaches: Vec<_> = colors
            .iter()
            .map(|attachment| {
                Some(wgpu::RenderPassColorAttachment {
                    view: attachment.view.get(),
                    depth_slice: None,
                    resolve_target: attachment.resolve_target.map(|target| target.get()),
                    ops: wgpu::Operations {
                        load: attachment.load_op,
                        store: attachment.store_op,
                    },
                })
            })
            .collect();

        self.color_attaches = attaches;

        self
    }

    pub fn with_depth(mut self, depth: Option<&RenderPassDepthAttach<'r>>) -> Self {
        let attach = depth.map(|attachment| wgpu::RenderPassDepthStencilAttachment {
            view: attachment.view.get(),

            depth_ops: Some(wgpu::Operations {
                load: attachment.depth_load_op,
                store: attachment.depth_store_op,
            }),

            stencil_ops: attachment.stencil_load_op.map(|load| wgpu::Operations {
                load,
                store: attachment
                    .stencil_store_op
                    .expect("stencil store op must be specified"),
            }),
        });

        self.depth_attach = attach;

        self
    }

    pub fn begin(self, encoder: &'r mut CommandEncoder) -> Self {
        Self {
            _pass: Some(
                encoder
                    .get_mut()
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Glumy Render Pass"),
                        color_attachments: &self.color_attaches,
                        depth_stencil_attachment: self.depth_attach,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                        multiview_mask: None,
                    }),
            ),
            color_attaches: Vec::new(),
            depth_attach: None,
        }
    }
}
