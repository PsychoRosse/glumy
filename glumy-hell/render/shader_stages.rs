use bitflags::bitflags;

#[cfg(feature = "clama")]
use clama::Event;

bitflags! {
    #[cfg_attr(feature = "clama", derive(Event))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ShaderStages: u32 {
        const NONE = 1 << 0;
        const VERTEX = 1 << 1;
        const FRAGMENT = 1 << 2;
        const COMPUTE = 1 << 3;
        const VERTEX_FRAGMENT = 1 << 4;
    }
}
