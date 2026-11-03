#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlumyHellError {
    SurfaceCreate,
    AdapterRequest,
    DeviceCreate,
}
