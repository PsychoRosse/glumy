use crate::{GpuInstance, InstanceError, WindowSurface};

pub struct Adapter {
    adapter: wgpu::Adapter,
}

impl Adapter {
    pub async fn new(instance: &GpuInstance, surface: &WindowSurface) -> Self {
        let adapter = instance
            .get()
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(surface.get()),
                apply_limit_buckets: false,
            })
            .await
            .unwrap_or_else(|_| panic!("{}", InstanceError::AdapterRequest));

        Self { adapter }
    }
}

impl Adapter {
    pub(crate) fn get(&self) -> &wgpu::Adapter {
        &self.adapter
    }
}
