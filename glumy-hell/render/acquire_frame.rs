use crate::{SurfaceTexture, WhatFrame};

pub enum AcquireFrame {
    Frame(Option<SurfaceTexture>, WhatFrame),
}

impl AcquireFrame {
    pub fn is_suboptimal(&self) -> bool {
        matches!(self, AcquireFrame::Frame(_, WhatFrame::Suboptimal))
    }

    pub fn is_success_suboptimal_take(self) -> Option<SurfaceTexture> {
        match self {
            AcquireFrame::Frame(texture, WhatFrame::Success | WhatFrame::Suboptimal) => texture,
            _ => None,
        }
    }
}
