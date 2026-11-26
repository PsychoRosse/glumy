use crate::Input;

use clama::{Plugin, Registry, Stage};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_res(Input::default())
            .with_system(Stage::PreInputUpdate, Input::pre_input_system)
            .with_system(Stage::PreInputUpdate, Input::update_input_time)
            .with_system(Stage::PostInputUpdate, Input::post_input_system);
    }
}
