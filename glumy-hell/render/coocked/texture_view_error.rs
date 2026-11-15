use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureViewError {
    #[error("Unsupported texture format")]
    UnsupportedTextureFormat,
}
