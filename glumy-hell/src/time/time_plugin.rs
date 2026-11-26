use crate::{Time, TimeDriver};

use clama::{Plugin, Registry, ResMut, Stage};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_res(Time::default())
            .with_res(TimeDriver::default())
            .with_system(Stage::TimeUpdate, time_system);
    }
}

#[inline(always)]
fn time_system(mut time: ResMut<Time>, mut driver: ResMut<TimeDriver>) {
    let (delta, elapsed) = driver.update();

    time.update(delta, elapsed);
}
