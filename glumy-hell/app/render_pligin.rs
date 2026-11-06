use crate::{
    Color, GraphicsDevice, RenderPass, RenderPassColorAttach, SurfaceTexture, WindowSurface,
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

            let mut encoder = device.create_encoder();

            {
                let attach = RenderPassColorAttach::clear(&view, Color::rgb(0.05, 0.05, 0.05));

                let _ = RenderPass::default()
                    .with_colors(&[attach])
                    .with_depth(None)
                    .begin(&mut encoder);
            }

            device.present(encoder, frame);
        }
    }
}
