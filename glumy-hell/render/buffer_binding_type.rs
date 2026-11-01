pub enum BufferBindingType {
    Uniform,
    Storage { read_only: bool },
}
