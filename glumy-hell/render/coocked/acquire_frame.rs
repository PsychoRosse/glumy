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

    pub fn is_validation(&self) -> bool {
        matches!(self, AcquireFrame::Frame(_, WhatFrame::Validation))
    }

    pub fn is_timeout(&self) -> bool {
        matches!(self, AcquireFrame::Frame(_, WhatFrame::Timeout))
    }

    pub fn is_outdated(&self) -> bool {
        matches!(self, AcquireFrame::Frame(_, WhatFrame::Outdated))
    }

    pub fn is_occluded(&self) -> bool {
        matches!(self, AcquireFrame::Frame(_, WhatFrame::Occluded))
    }

    pub fn is_lost(&self) -> bool {
        matches!(self, AcquireFrame::Frame(_, WhatFrame::Lost))
    }
}
