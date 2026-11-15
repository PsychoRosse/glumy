use crate::{TextureAspect, TextureFormat, TextureViewDimension, TextureViewError};

#[derive(Debug, Clone)]
pub struct TextureViewDescriptor {
    pub label: Option<String>,
    pub format: Option<TextureFormat>,
    pub dimension: Option<TextureViewDimension>,
    pub aspect: TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<u32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<u32>,
}

impl TextureViewDescriptor {
    pub(crate) fn to_wgpu(&self) -> wgpu::TextureViewDescriptor<'_> {
        wgpu::TextureViewDescriptor {
            label: self.label.as_deref(),
            format: self.format.map(TextureFormat::to_wgpu),
            dimension: self.dimension.map(TextureViewDimension::to_wgpu),
            aspect: self.aspect.to_wgpu(),
            base_mip_level: self.base_mip_level,
            mip_level_count: self.mip_level_count,
            base_array_layer: self.base_array_layer,
            array_layer_count: self.array_layer_count,
            usage: None,
        }
    }

    pub(crate) fn from_wgpu(
        descriptor: &wgpu::TextureViewDescriptor<'_>,
    ) -> Result<Self, TextureViewError> {
        Ok(Self {
            label: descriptor.label.map(String::from),

            format: descriptor
                .format
                .map(TextureFormat::from_wgpu)
                .transpose()
                .map_err(|_| TextureViewError::UnsupportedTextureFormat)?,

            dimension: descriptor
                .dimension
                .map(|dimension| TextureViewDimension::from_wgpu(dimension)),

            aspect: TextureAspect::from_wgpu(descriptor.aspect),

            base_mip_level: descriptor.base_mip_level,
            mip_level_count: descriptor.mip_level_count,
            base_array_layer: descriptor.base_array_layer,
            array_layer_count: descriptor.array_layer_count,
        })
    }
}
