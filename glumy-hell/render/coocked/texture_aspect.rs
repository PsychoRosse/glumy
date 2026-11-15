#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureAspect {
    All,
    StencilOnly,
    DepthOnly,
    Plane0,
    Plane1,
    Plane2,
}

impl TextureAspect {
    pub(crate) fn to_wgpu(&self) -> wgpu::TextureAspect {
        match self {
            Self::All => wgpu::TextureAspect::All,
            Self::StencilOnly => wgpu::TextureAspect::StencilOnly,
            Self::DepthOnly => wgpu::TextureAspect::DepthOnly,
            Self::Plane0 => wgpu::TextureAspect::Plane0,
            Self::Plane1 => wgpu::TextureAspect::Plane1,
            Self::Plane2 => wgpu::TextureAspect::Plane2,
        }
    }

    pub(crate) fn from_wgpu(aspect: wgpu::TextureAspect) -> Self {
        match aspect {
            wgpu::TextureAspect::All => Self::All,
            wgpu::TextureAspect::StencilOnly => Self::StencilOnly,
            wgpu::TextureAspect::DepthOnly => Self::DepthOnly,
            wgpu::TextureAspect::Plane0 => Self::Plane0,
            wgpu::TextureAspect::Plane1 => Self::Plane1,
            wgpu::TextureAspect::Plane2 => Self::Plane2,
        }
    }
}
