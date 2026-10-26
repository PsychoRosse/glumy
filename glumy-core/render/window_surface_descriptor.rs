use crate::{Adapter, GpuInstance, SurfaceTarget};

use plam::SurfaceSize;

pub struct WindowSurfaceDescriptor<'d> {
    pub instance: &'d GpuInstance,
    pub adapter: &'d Adapter,
    pub surface_size: SurfaceSize,
    pub surface_target: SurfaceTarget,
}
