use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstanceError {
    #[error("Failed to create surface")]
    SurfaceCreate,
    #[error("Failed to request adapter")]
    AdapterRequest,
    #[error("Failed to create device")]
    DeviceCreate,
}
