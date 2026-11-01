use crate::{GpuInstance, GraphicsDevice, WindowSurface};

pub struct GlumyHell {
    pub instance: GpuInstance,
    pub surface: WindowSurface,
    pub device: GraphicsDevice,
}
