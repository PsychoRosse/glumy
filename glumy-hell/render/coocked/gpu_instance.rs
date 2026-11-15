use crate::RawGpuInstance;

#[derive(Debug)]
pub struct GpuInstance {
    raw: RawGpuInstance,
}

impl GpuInstance {
    pub fn raw(&self) -> &RawGpuInstance {
        &self.raw
    }
}

impl GpuInstance {
    pub(crate) fn from_raw(raw: RawGpuInstance) -> Self {
        Self { raw }
    }
}
