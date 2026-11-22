pub trait WindowFrontend {
    fn redraw(&self);

    fn size(&self) -> plam::SurfaceSize;
}
