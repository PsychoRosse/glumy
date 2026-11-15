#[derive(Debug)]
pub struct RawGpuInstance {
    instance: wgpu::Instance,
}

impl Default for RawGpuInstance {
    fn default() -> Self {
        Self {
            instance: wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::DX12,
                flags: wgpu::InstanceFlags::default(),
                memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
                backend_options: wgpu::BackendOptions::default(),
                display: None,
            }),
        }
    }
}

impl RawGpuInstance {
    #[inline]
    pub(crate) fn raw_get(&self) -> &wgpu::Instance {
        &self.instance
    }
}
