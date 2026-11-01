use crate::SurfaceTexture;

#[derive(Debug, Clone)]
pub enum CurrentSurfaceTexture {
    Success(SurfaceTexture),
    Suboptimal(SurfaceTexture),
    Timeout,
    Occluded,
    Outdated,
    Lost,
    Validation,
}
