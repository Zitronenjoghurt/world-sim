use crate::components::growth_timer::GrowthTimer;
use bevy::prelude::Component;
use rand::{thread_rng, Rng};
use std::time::Duration;

#[derive(Component)]
pub struct NaturalGrassGrowth {
    pub growth_timer: GrowthTimer,
    pub growth_chance: f32,
}

impl NaturalGrassGrowth {
    pub fn new(
        growth_seconds: f32,
        growth_chance: f32,
    ) -> Self {
        Self {
            growth_timer: GrowthTimer::from_seconds(growth_seconds),
            growth_chance,
        }
    }

    pub fn tick(&mut self, duration: Duration) -> bool {
        self.growth_timer.tick(duration);

        if self.growth_timer.just_finished() {
            let mut rng = thread_rng();
            return rng.gen::<f32>() <= self.growth_chance;
        }

        false
    }
}