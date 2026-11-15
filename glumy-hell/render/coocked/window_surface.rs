use crate::{
    AcquireFrame, RawCurrentSurfaceTexture, RawGraphicsDevice, RawWindowSurface, SurfaceTexture,
    WhatFrame,
};

#[derive(Debug)]
pub struct WindowSurface {
    raw: RawWindowSurface,
}

impl WindowSurface {
    pub fn acquire_frame(&self) -> AcquireFrame {
        match self.raw.acquire_frame() {
            RawCurrentSurfaceTexture::Success(frame) => {
                AcquireFrame::Frame(Some(SurfaceTexture::from_raw(frame)), WhatFrame::Success)
            }
            RawCurrentSurfaceTexture::Suboptimal(frame) => {
                AcquireFrame::Frame(Some(SurfaceTexture::from_raw(frame)), WhatFrame::Suboptimal)
            }
            RawCurrentSurfaceTexture::Validation => {
                AcquireFrame::Frame(None, WhatFrame::Validation)
            }
            RawCurrentSurfaceTexture::Timeout => AcquireFrame::Frame(None, WhatFrame::Timeout),
            RawCurrentSurfaceTexture::Occluded => AcquireFrame::Frame(None, WhatFrame::Occluded),
            RawCurrentSurfaceTexture::Outdated => AcquireFrame::Frame(None, WhatFrame::Outdated),
            RawCurrentSurfaceTexture::Lost => AcquireFrame::Frame(None, WhatFrame::Lost),
        }
    }

    pub fn configure(&self, device: &RawGraphicsDevice) {
        self.raw.configure(device);
    }
}

impl WindowSurface {
    pub(crate) fn from_raw(raw: RawWindowSurface) -> Self {
        Self { raw }
    }
}
