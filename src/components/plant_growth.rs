use bevy::prelude::{Component, Timer};

#[derive(Component)]
pub struct PlantGrowth {
    pub growth_stage: u32,
    pub max_growth_stage: u32,
    pub growth_timer: Timer,
}