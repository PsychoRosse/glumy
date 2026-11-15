use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HellError {
    #[error("Failed to create surface")]
    SurfaceCreate,
    #[error("Failed to request adapter")]
    AdapterRequest,
    #[error("Failed to create device")]
    DeviceCreate,
}
