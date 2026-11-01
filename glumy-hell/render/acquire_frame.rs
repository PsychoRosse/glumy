use crate::{SurfaceTexture, WhatFrame};

pub enum AcquireFrame {
    Frame(Option<SurfaceTexture>, WhatFrame),
}
