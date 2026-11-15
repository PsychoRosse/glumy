use glumy::{
    Application, Color, EventLoopError, GpuInstance, GraphicsDevice, Hell, HellDescriptor, RenderPassColorAttachment, RenderPassDepthAttachment, WindowApplication, WindowAttributes, WindowEvent, WindowFrontend, WindowHandle, WindowId, WindowSurface,
};

use pollster::block_on;

fn main() -> Result<(), EventLoopError> {
    HellApp::default().run_from(WindowAttributes::default())
}

#[derive(Default)]
pub struct HellApp {
    instance: Option<GpuInstance>,
    device: Option<GraphicsDevice>,
    surface: Option<WindowSurface>,
}

impl HellApp {
    pub fn run_from(self, attr: WindowAttributes) -> Result<(), EventLoopError> {
        WindowApplication::new(attr, self).run()
    }
}

impl Application for HellApp {
    fn start(&mut self, window: &impl WindowFrontend, handle: &impl WindowHandle) {
        let hell = block_on(Hell::enter(HellDescriptor {
            surface_target: handle.target().unwrap(),
            surface_size: window.size(),
        }))
        .unwrap();

        self.instance = Some(hell.instance);
        self.device = Some(hell.device);
        self.surface = Some(hell.surface);
    }

    fn event(&mut self, event: WindowEvent, _: WindowId) {
        let Some(surface) = &mut self.surface else {
            return;
        };

        let Some(device) = &mut self.device else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                let acquire_frame = surface.acquire_frame();

                if acquire_frame.is_suboptimal() {
                    surface.configure(device.raw());
                }

                if let Some(frame) = acquire_frame.is_success_suboptimal_take() {
                    let view = frame.create_default_view();
                    let clear_color = Color::rgb(0.05, 0.05, 0.05);

                    let mut encoder = device.create_encoder();

                    let color_attach = &[RenderPassColorAttachment::new(&view, None)];
                    let depth_attach = RenderPassDepthAttachment::clear(&view, 1.0);

                    let render_pass = RenderPass

                    device.present(encoder, frame);
                }
            }
            _ => {}
        }
    }
}
