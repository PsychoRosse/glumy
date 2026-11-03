use crate::{GpuInstance, GraphicsDevice, WindowSurface};

#[derive(Debug)]
pub struct GlumyHell {
    pub instance: GpuInstance,
    pub surface: WindowSurface,
    pub device: GraphicsDevice,
}
