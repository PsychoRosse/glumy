use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ShaderStages: u32 {
        const NONE = 1 << 0;
        const VERTEX = 1 << 1;
        const FRAGMENT = 1 << 2;
        const COMPUTE = 1 << 3;
        const VERTEX_FRAGMENT = 1 << 4;
    }
}
