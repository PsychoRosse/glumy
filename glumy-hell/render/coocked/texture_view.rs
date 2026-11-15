use crate::RawTextureView;

pub struct TextureView {
    raw: RawTextureView,
}

impl TextureView {
    pub fn from_raw(raw: RawTextureView) -> Self {
        Self { raw }
    }

    pub fn raw_get(&self) -> &RawTextureView {
        &self.raw
    }
}

impl TextureView {
    pub(crate) fn from_wgpu(raw: wgpu::TextureView) -> Self {
        Self {
            raw: RawTextureView::from_wgpu(raw),
        }
    }
}
