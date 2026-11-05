#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> Self {
        let array = color.value;

        Self {
            r: array[0],
            g: array[1],
            b: array[2],
            a: array[3],
        }
    }
}
