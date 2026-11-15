use crate::{
    GpuInstance, GraphicsDevice, HellDescriptor, HellError, HellSetup, RawGpuInstance,
    RawGraphicsDevice, RawWindowSurface, WindowSurface,
};

pub struct Hell;

impl Hell {
    pub async fn enter(descriptor: HellDescriptor) -> Result<HellSetup, HellError> {
        let instance = RawGpuInstance::default();
        let mut surface = RawWindowSurface::new(&instance, descriptor.surface_target)?;
        let device = RawGraphicsDevice::new(&instance, &surface).await?;
        let config = device.raw_config(&surface, &device, descriptor.surface_size);

        surface.set_config(config);
        surface.configure(&device);

        Ok(HellSetup {
            instance: GpuInstance::from_raw(instance),
            surface: WindowSurface::from_raw(surface),
            device: GraphicsDevice::from_raw(device),
        })
    }
}
