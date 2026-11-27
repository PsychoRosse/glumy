use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    time::Duration,
};

#[derive(Debug)]
pub struct DeviceState<D: Copy + Eq + Hash> {
    pressed: HashSet<D>,
    just_pressed: HashSet<D>,
    just_released: HashSet<D>,
    held_for: HashMap<D, Duration>,
}

impl<D: Copy + Eq + Hash> Default for DeviceState<D> {
    fn default() -> Self {
        Self {
            pressed: HashSet::default(),
            just_pressed: HashSet::default(),
            just_released: HashSet::default(),
            held_for: HashMap::default(),
        }
    }
}

impl<D: Copy + Eq + Hash> DeviceState<D> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            pressed: HashSet::with_capacity(capacity),
            just_pressed: HashSet::with_capacity(capacity),
            just_released: HashSet::with_capacity(capacity),
            held_for: HashMap::with_capacity(capacity),
        }
    }
}

impl<D: Copy + Eq + Hash> DeviceState<D> {
    #[inline]
    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    #[inline]
    pub fn contains_all(&self, buttons: &[D]) -> bool {
        buttons.iter().all(|button| self.pressed.contains(button))
    }

    #[inline]
    pub fn press(&mut self, button: D) {
        if self.pressed.insert(button) {
            self.just_pressed.insert(button);
            self.held_for.insert(button, Duration::ZERO);
        }
    }

    #[inline]
    pub fn release(&mut self, button: D) {
        if self.pressed.remove(&button) {
            self.just_released.insert(button);
            self.held_for.remove(&button);
        }
    }

    #[inline]
    pub fn is_pressed(&self, button: D) -> bool {
        self.pressed.contains(&button)
    }

    #[inline]
    pub fn is_just_pressed(&self, button: D) -> bool {
        self.just_pressed.contains(&button)
    }

    #[inline]
    pub fn is_just_released(&self, button: D) -> bool {
        self.just_released.contains(&button)
    }

    #[inline]
    pub fn is_held_for(&self, button: D, duration: Duration) -> bool {
        self.held_for
            .get(&button)
            .is_some_and(|elapsed| *elapsed >= duration)
    }

    #[inline]
    pub fn update(&mut self, delta: f32) {
        if !delta.is_finite() || delta <= 0.0 {
            return;
        }

        let delta = Duration::from_secs_f32(delta);

        for elapsed in self.held_for.values_mut() {
            *elapsed += delta;
        }
    }
}
