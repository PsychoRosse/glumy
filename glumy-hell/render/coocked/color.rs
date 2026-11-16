#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub value: [f64; 4],
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Color {
    pub const WHITE: Self = Self::rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    pub const RED: Self = Self::rgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::rgba(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::rgba(0.0, 0.0, 1.0, 1.0);
}

impl Color {
    pub const fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            value: [r, g, b, 1.0],
        }
    }

    pub const fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            value: [r, g, b, a],
        }
    }
}

impl Color {
    pub(crate) fn to_wgpu(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.value[0],
            g: self.value[1],
            b: self.value[2],
            a: self.value[3],
        }
    }
}
