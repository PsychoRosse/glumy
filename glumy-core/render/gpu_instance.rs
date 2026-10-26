#[cfg(feature = "clama")]
use clama::Resource;

#[cfg_attr(feature = "clama", derive(Resource))]
pub struct GpuInstance {
    instance: wgpu::Instance,
}

impl Default for GpuInstance {
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

impl GpuInstance {
    pub(crate) fn get(&self) -> &wgpu::Instance {
        &self.instance
    }
}
