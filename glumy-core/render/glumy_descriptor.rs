use crate::{Adapter, GpuInstance, SurfaceTarget};

use plam::SurfaceSize;

pub struct GlumyDescriptor {
    pub surface_size: SurfaceSize,
    pub surface_target: SurfaceTarget,
}
