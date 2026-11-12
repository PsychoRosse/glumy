use crate::{FrameLatency, SurfaceTarget, VerticalSync};

use plam::SurfaceSize;

pub struct GlumyDescriptor {
    pub surface_target: SurfaceTarget,
    pub surface_size: SurfaceSize,
    pub vertical_sync: VerticalSync,
    pub frame_latency: FrameLatency,
}
