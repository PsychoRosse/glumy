#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalSync {
    #[default]
    On,
    Mailbox,
    Immediate,
}

impl From<VerticalSync> for wgpu::PresentMode {
    fn from(mode: VerticalSync) -> Self {
        match mode {
            VerticalSync::On => Self::Fifo,
            VerticalSync::Mailbox => Self::Mailbox,
            VerticalSync::Immediate => Self::Immediate,
        }
    }
}
