use crate::components::enums::plant_type::PlantType;
use crate::components::growth_timer::GrowthTimer;
use bevy::prelude::Component;
use std::time::Duration;

#[derive(Component)]
pub struct Plant {
    plant_type: PlantType,
    growth_timer: GrowthTimer,
    growth_stage: usize,
    max_growth_stage: usize,
}

impl Plant {
    pub fn new(
        plant_type: PlantType,
        growth_stages_count: usize,
        growth_time: f32,
    ) -> Self {
        Self {
            plant_type,
            growth_timer: GrowthTimer::from_seconds(growth_time),
            growth_stage: 0,
            max_growth_stage: growth_stages_count.saturating_sub(1),
        }
    }

    pub fn get_growth_stage(&self) -> usize {
        self.growth_stage
    }

    pub fn finished_growing(&self) -> bool {
        self.growth_stage == self.max_growth_stage
    }

    pub fn tick_growth(&mut self, delta: Duration) -> bool {
        if self.finished_growing() {
            return false;
        }

        self.growth_timer.tick(delta);
        if self.growth_timer.just_finished() {
            self.growth_stage = self.growth_stage.saturating_add(1);
            true
        } else {
            false
        }
    }
}