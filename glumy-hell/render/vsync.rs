#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vsync {
    #[default]
    On,
    Mailbox,
    Immediate,
}

impl From<Vsync> for wgpu::PresentMode {
    fn from(mode: Vsync) -> Self {
        match mode {
            Vsync::On => Self::Fifo,
            Vsync::Mailbox => Self::Mailbox,
            Vsync::Immediate => Self::Immediate,
        }
    }
}
