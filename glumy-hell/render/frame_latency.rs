#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameLatency {
    Zero,
    One,
    Two,
    Three,
}

impl FrameLatency {
    pub fn get_number(self) -> u32 {
        match self {
            FrameLatency::Zero => 0,
            FrameLatency::One => 1,
            FrameLatency::Two => 2,
            FrameLatency::Three => 3,
        }
    }
}
