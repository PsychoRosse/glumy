use crate::{Axis, AxisBinding, Input, Key};

use std::collections::HashMap;

pub struct ActionMap {
    axes: HashMap<Axis, AxisBinding>,
}

impl Default for ActionMap {
    fn default() -> Self {
        let axes = HashMap::from([
            (Axis::MoveX, AxisBinding::new(Key::A, Key::D)),
            (Axis::MoveY, AxisBinding::new(Key::W, Key::S)),
            (Axis::MoveZ, AxisBinding::new(Key::J, Key::K)),
            (Axis::LookX, AxisBinding::new(Key::H, Key::L)),
            (Axis::LookY, AxisBinding::new(Key::Q, Key::E)),
        ]);

        Self { axes }
    }
}

impl ActionMap {
    #[inline]
    pub fn axis(&self, axis: Axis, input: &Input) -> f32 {
        let Some(binding) = self.axes.get(&axis) else {
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
