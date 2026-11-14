use crate::{
    Color, GraphicsDevice, RenderPass, RenderPassColorAttach, RenderPassDescriptor, SurfaceTexture,
    WindowSurface,
};

use clama::{Plugin, Registry, Res, ResMut, Stage};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry.with_system(Stage::Render, Self::render_system);
    }
}

impl RenderPlugin {
    fn render_system(surface: ResMut<WindowSurface>, device: Res<GraphicsDevice>) {
        let acquire_frame = surface.acquire_frame();

        if acquire_frame.is_suboptimal() {
            surface.configure(&device);
        }

        if let Some(frame) = acquire_frame.is_success_suboptimal_take() {
            let view = SurfaceTexture::create_view(&frame);
            let color = Color::rgb(0.05, 0.05, 0.05);

            let mut encoder = device.create_encoder();

            let pass_desc = RenderPassDescriptor {
                color_attaches: &[RenderPassColorAttach::clear(&view, color)],
                depth_attach: None,
            };

            let _ = RenderPass::begin(&mut encoder, pass_desc, |_| {});

            device.present(encoder, frame);
        }
    }
}
