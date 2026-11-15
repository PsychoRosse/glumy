use crate::RawSurfaceTexture;

#[derive(Debug, Clone)]
pub enum RawCurrentSurfaceTexture {
    Success(RawSurfaceTexture),
    Suboptimal(RawSurfaceTexture),
    Timeout,
    Occluded,
    Outdated,
    Lost,
    Validation,
}

impl RawCurrentSurfaceTexture {
    pub(crate) fn from_wgpu(texture: wgpu::CurrentSurfaceTexture) -> Self {
        match texture {
            wgpu::CurrentSurfaceTexture::Success(texture) => {
                Self::Success(RawSurfaceTexture::from_wgpu(texture))
            }

            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                Self::Suboptimal(RawSurfaceTexture::from_wgpu(texture))
            }

            wgpu::CurrentSurfaceTexture::Timeout => Self::Timeout,
            wgpu::CurrentSurfaceTexture::Occluded => Self::Occluded,
            wgpu::CurrentSurfaceTexture::Outdated => Self::Outdated,
            wgpu::CurrentSurfaceTexture::Lost => Self::Lost,
            wgpu::CurrentSurfaceTexture::Validation => Self::Validation,
        }
    }

    pub(crate) fn to_wgpu(self) -> wgpu::CurrentSurfaceTexture {
        match self {
            Self::Success(texture) => wgpu::CurrentSurfaceTexture::Success(texture.raw_take()),

            Self::Suboptimal(texture) => wgpu::CurrentSurfaceTexture::Suboptimal(texture.raw_take()),

            Self::Timeout => wgpu::CurrentSurfaceTexture::Timeout,
            Self::Occluded => wgpu::CurrentSurfaceTexture::Occluded,
            Self::Outdated => wgpu::CurrentSurfaceTexture::Outdated,
            Self::Lost => wgpu::CurrentSurfaceTexture::Lost,
            Self::Validation => wgpu::CurrentSurfaceTexture::Validation,
        }
    }
}
