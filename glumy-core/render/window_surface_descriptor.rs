use crate::{GpuInstance, GraphicsDevice, SurfaceTarget};

use plam::SurfaceSize;

pub struct WindowSurfaceDescriptor<'d> {
    pub instance: &'d GpuInstance,
    pub device: &'d GraphicsDevice,
    pub surface_size: SurfaceSize,
    pub surface_target: SurfaceTarget,
}
