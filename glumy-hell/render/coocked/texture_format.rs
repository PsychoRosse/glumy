use crate::CurrentSurfaceTextureError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    R8Unorm,
    Rg8Unorm,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    R16Float,
    Rg16Float,
    Rgba16Float,
    R32Float,
    Rg32Float,
    Rgba32Float,
    Depth16Unorm,
    Depth24Plus,
    Depth24PlusStencil8,
    Depth32Float,
    Depth32FloatStencil8,
}

impl TextureFormat {
    pub(crate) fn from_wgpu(
        format: wgpu::TextureFormat,
    ) -> Result<Self, CurrentSurfaceTextureError> {
        match format {
            wgpu::TextureFormat::R8Unorm => Ok(Self::R8Unorm),
            wgpu::TextureFormat::Rg8Unorm => Ok(Self::Rg8Unorm),
            wgpu::TextureFormat::Rgba8Unorm => Ok(Self::Rgba8Unorm),
            wgpu::TextureFormat::Rgba8UnormSrgb => Ok(Self::Rgba8UnormSrgb),
            wgpu::TextureFormat::Bgra8Unorm => Ok(Self::Bgra8Unorm),
            wgpu::TextureFormat::Bgra8UnormSrgb => Ok(Self::Bgra8UnormSrgb),
            wgpu::TextureFormat::R16Float => Ok(Self::R16Float),
            wgpu::TextureFormat::Rg16Float => Ok(Self::Rg16Float),
            wgpu::TextureFormat::Rgba16Float => Ok(Self::Rgba16Float),
            wgpu::TextureFormat::R32Float => Ok(Self::R32Float),
            wgpu::TextureFormat::Rg32Float => Ok(Self::Rg32Float),
            wgpu::TextureFormat::Rgba32Float => Ok(Self::Rgba32Float),
            wgpu::TextureFormat::Depth16Unorm => Ok(Self::Depth16Unorm),
            wgpu::TextureFormat::Depth24Plus => Ok(Self::Depth24Plus),
            wgpu::TextureFormat::Depth24PlusStencil8 => Ok(Self::Depth24PlusStencil8),
            wgpu::TextureFormat::Depth32Float => Ok(Self::Depth32Float),
            wgpu::TextureFormat::Depth32FloatStencil8 => Ok(Self::Depth32FloatStencil8),
            _ => Err(CurrentSurfaceTextureError::UnsupportedTextureFormat),
        }
    }

    pub(crate) fn to_wgpu(self) -> wgpu::TextureFormat {
        match self {
            Self::R8Unorm => wgpu::TextureFormat::R8Unorm,
            Self::Rg8Unorm => wgpu::TextureFormat::Rg8Unorm,
            Self::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            Self::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
            Self::Bgra8Unorm => wgpu::TextureFormat::Bgra8Unorm,
            Self::Bgra8UnormSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
            Self::R16Float => wgpu::TextureFormat::R16Float,
            Self::Rg16Float => wgpu::TextureFormat::Rg16Float,
            Self::Rgba16Float => wgpu::TextureFormat::Rgba16Float,
            Self::R32Float => wgpu::TextureFormat::R32Float,
            Self::Rg32Float => wgpu::TextureFormat::Rg32Float,
            Self::Rgba32Float => wgpu::TextureFormat::Rgba32Float,
            Self::Depth16Unorm => wgpu::TextureFormat::Depth16Unorm,
            Self::Depth24Plus => wgpu::TextureFormat::Depth24Plus,
            Self::Depth24PlusStencil8 => wgpu::TextureFormat::Depth24PlusStencil8,
            Self::Depth32Float => wgpu::TextureFormat::Depth32Float,
            Self::Depth32FloatStencil8 => wgpu::TextureFormat::Depth32FloatStencil8,
        }
    }
}
