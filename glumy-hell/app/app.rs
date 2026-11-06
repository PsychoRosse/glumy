use crate::{
    Application, EventLoopError, Glumy, GlumyDescriptor, Latency, Vsync, WindowApplication,
    WindowAttributes, WindowEvent, WindowFrontend, WindowHandle, WindowId,
};

use clama::{Plugin, PluginRegistry, Registry, Scheduler, World, WorldCell};

#[derive(Default)]
pub struct App {
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
