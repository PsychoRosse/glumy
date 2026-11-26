use clama::Resource;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Resource, Debug, Clone, Copy)]
pub struct Time {
    delta: f32,
    elapsed: f32,
    max_delta: f32,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            delta: 0.0,
            elapsed: 0.0,
            max_delta: Self::DEFAULT_MAX_DELTA,
        }
    }
}

impl Time {
    pub const DEFAULT_MAX_DELTA: f32 = 0.2;
}

impl Time {
    #[inline]
    pub(crate) fn update(&mut self, raw_delta: f32, elapsed: f32) {
        self.delta = raw_delta.min(self.max_delta);
        self.elapsed = elapsed;
    }

    #[inline]
    #[must_use]
    pub fn delta(&self) -> f32 {
        self.delta
    }

    #[inline]
    #[must_use]
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    #[inline]
    #[must_use]
    pub fn max_delta(&self) -> f32 {
        self.max_delta
    }
}

impl Time {
    pub fn set_max_delta(&mut self, max_delta: f32) {
        self.max_delta = max_delta;
        self.delta = self.delta.min(max_delta);
    }
}
