use crate::Color;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Material {
    color: Color,
}

impl Material {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}

impl From<Color> for Material {
    fn from(color: Color) -> Self {
        Self::new(color)
    }
}
