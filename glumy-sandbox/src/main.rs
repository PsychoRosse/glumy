use clama::{Plugin, PluginRegistry, Registry, Res, ResMut, Scheduler, Stage, World, WorldCell};

use glumy::{
    Application, Color, EventLoopError, Glumy, GlumyDescriptor, GraphicsDevice, Latency,
    RenderPass, RenderPassColorAttach, RenderPassDepthAttach, SurfaceTexture, Vsync,
    WindowApplication, WindowAttributes, WindowEvent, WindowFrontend, WindowHandle, WindowId,
    WindowSurface,
};

fn main() -> Result<(), EventLoopError> {
    App::default()
        .with_plugin(RenderPlugin)
        .run_from(WindowAttributes::default())
}

#[derive(Default)]
struct App {
    world: World,
    plugins: PluginRegistry,
    scheduler: Scheduler,
}

impl App {
    pub fn run_from(mut self, attr: WindowAttributes) -> Result<(), EventLoopError> {
        self.plugins
            .build(WorldCell::from(&mut self.world), &mut self.scheduler);

        WindowApplication::new(attr, self).run()
    }

    pub fn with_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        self.plugins.with_plugin(plugin);
        self
    }
}

impl Application for App {
    fn start(&mut self, window: &impl WindowFrontend, handle: &impl WindowHandle) {
        let glumy = Glumy::create(GlumyDescriptor {
            surface_target: handle.target().unwrap(),
            surface_size: window.size(),
            vsync: Vsync::default(),
            frame_latency: Latency::Two,
        })
        .unwrap();

        self.world
            .with_res(glumy.instance)
            .with_res(glumy.surface)
            .with_res(glumy.device);

        self.scheduler.run_startup(WorldCell::from(&mut self.world));
    }

    fn event(&mut self, event: WindowEvent, _: WindowId) {
        let cell = WorldCell::from(&mut self.world);

        if event == WindowEvent::RedrawRequested {
            cell.update();

            self.scheduler.run_full(cell, 0.0);
        }
    }
}

struct RenderPlugin;

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
