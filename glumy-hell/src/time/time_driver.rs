use {
    clama::Resource,
    std::time::{Duration, Instant},
};

#[derive(Resource, Debug)]
pub struct TimeDriver {
    startup: Instant,
    last: Instant,
}

impl Default for TimeDriver {
    fn default() -> Self {
        let now = Instant::now();

        Self {
            startup: now,
            last: now,
        }
    }
}

impl TimeDriver {
    #[inline]
    pub(crate) fn update(&mut self) -> (f32, f32) {
        let now = Instant::now();

        let delta = now
            .checked_duration_since(self.last)
            .unwrap_or(Duration::ZERO);

        let elapsed = now
            .checked_duration_since(self.startup)
            .unwrap_or(Duration::ZERO);

        self.last = now;

        (delta.as_secs_f32(), elapsed.as_secs_f32())
    }
}
