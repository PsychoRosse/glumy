use thiserror::Error;

#[derive(Error, Debug)]
pub enum CurrentSurfaceTextureError {
    #[error("Unsupported glumy texture format")]
    UnsupportedTextureFormat,
    #[error("Surface Texture Is None")]
    SurfaceTextureNotCreated,
}
