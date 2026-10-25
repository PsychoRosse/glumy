use plam::SurfaceSize;

pub trait WindowFrontend {
    fn redraw(&self);

    fn size(&self) -> SurfaceSize;
}
