#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Latency {
    Zero,
    One,
    Two,
    Three,
}

impl Latency {
    pub fn get_number(self) -> u32 {
        match self {
            Latency::Zero => 0,
            Latency::One => 1,
            Latency::Two => 2,
            Latency::Three => 3,
        }
    }
}
