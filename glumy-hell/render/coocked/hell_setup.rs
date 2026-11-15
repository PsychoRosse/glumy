use crate::{GpuInstance, GraphicsDevice, WindowSurface};

#[derive(Debug)]
pub struct HellSetup {
    pub instance: GpuInstance,
    pub device: GraphicsDevice,
    pub surface: WindowSurface,
}
