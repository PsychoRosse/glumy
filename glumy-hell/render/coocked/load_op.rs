use crate::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadOp<L> {
    Load,
    Clear(L),
}

impl LoadOp<u32> {
    pub(crate) fn to_wgpu_u32(self) -> wgpu::LoadOp<u32> {
        match self {
            Self::Load => wgpu::LoadOp::Load,
            Self::Clear(value) => wgpu::LoadOp::Clear(value),
        }
    }
}

impl LoadOp<f32> {
    pub(crate) fn to_wgpu_f32(self) -> wgpu::LoadOp<f32> {
        match self {
            Self::Load => wgpu::LoadOp::Load,
            Self::Clear(value) => wgpu::LoadOp::Clear(value),
        }
    }
}

impl LoadOp<Color> {
    pub(crate) fn to_wgpu_color(self) -> wgpu::LoadOp<wgpu::Color> {
        match self {
            Self::Load => wgpu::LoadOp::Load,
            Self::Clear(color) => wgpu::LoadOp::Clear(color.to_wgpu()),
        }
    }
}

impl<T> LoadOp<T> {
    pub(crate) fn to_wgpu(self, map: impl FnOnce(T) -> T) -> wgpu::LoadOp<T> {
        match self {
            Self::Load => wgpu::LoadOp::Load,
            Self::Clear(value) => wgpu::LoadOp::Clear(map(value)),
        }
    }
}
