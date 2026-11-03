#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vsync {
    #[default]
    Default,
    Mailbox,
    Immediate,
}

impl From<Vsync> for wgpu::PresentMode {
    fn from(mode: Vsync) -> Self {
        match mode {
            Vsync::Default => Self::Fifo,
            Vsync::Mailbox => Self::Mailbox,
            Vsync::Immediate => Self::Immediate,
        }
    }
}
