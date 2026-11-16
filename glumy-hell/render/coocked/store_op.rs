pub enum StoreOp {
    Store,
    Discard,
}

impl StoreOp {
    pub(crate) fn to_wgpu(self) -> wgpu::StoreOp {
        match self {
            Self::Store => wgpu::StoreOp::Store,
            Self::Discard => wgpu::StoreOp::Discard,
        }
    }
}
