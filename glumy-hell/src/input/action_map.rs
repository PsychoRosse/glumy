use crate::{Axis, AxisBinding, Input, Key};

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ActionMap {
    axis: HashMap<Axis, AxisBinding>,
}

impl Default for ActionMap {
    fn default() -> Self {
        let axis = Self::create_binding();

        Self { axis }
    }
}

impl ActionMap {
    #[inline]
    pub fn axis(&self, axis: Axis, input: &Input) -> f32 {
        let Some(binding) = self.axis.get(&axis) else {
            return 0.0;
        };

        let mut value = 0.0;

        if input.key_pressed(binding.positive()) {
            value += 1.0;
        }

        if input.key_pressed(binding.negative()) {
            value -= 1.0;
        }

        value
    }
}

impl ActionMap {
    fn create_binding() -> HashMap<Axis, AxisBinding> {
        HashMap::from([
            (Axis::MoveX, AxisBinding::new(Key::A, Key::D)),
            (Axis::MoveY, AxisBinding::new(Key::W, Key::S)),
            (Axis::MoveZ, AxisBinding::new(Key::J, Key::K)),
            (Axis::LookX, AxisBinding::new(Key::H, Key::L)),
            (Axis::LookY, AxisBinding::new(Key::Q, Key::E)),
        ])
    }
}
