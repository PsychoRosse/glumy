use crate::{Latency, SurfaceTarget, Vsync};

use plam::SurfaceSize;

pub struct GlumyDescriptor {
    pub surface_target: SurfaceTarget,
    pub surface_size: SurfaceSize,
    pub vsync: Vsync,
    pub frame_latency: Latency,
}
