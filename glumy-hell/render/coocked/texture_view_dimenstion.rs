#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureViewDimension {
    D1,
    D2,
    D2Array,
    Cube,
    CubeArray,
    D3,
}

impl TextureViewDimension {
    pub(crate) fn to_wgpu(self) -> wgpu::TextureViewDimension {
        match self {
            Self::D1 => wgpu::TextureViewDimension::D1,
            Self::D2 => wgpu::TextureViewDimension::D2,
            Self::D2Array => wgpu::TextureViewDimension::D2Array,
            Self::Cube => wgpu::TextureViewDimension::Cube,
            Self::CubeArray => wgpu::TextureViewDimension::CubeArray,
            Self::D3 => wgpu::TextureViewDimension::D3,
        }
    }

    pub(crate) fn from_wgpu(dimension: wgpu::TextureViewDimension) -> Self {
        match dimension {
            wgpu::TextureViewDimension::D1 => Self::D1,
            wgpu::TextureViewDimension::D2 => Self::D2,
            wgpu::TextureViewDimension::D2Array => Self::D2Array,
            wgpu::TextureViewDimension::Cube => Self::Cube,
            wgpu::TextureViewDimension::CubeArray => Self::CubeArray,
            wgpu::TextureViewDimension::D3 => Self::D3,
        }
    }
}
