use crate::components::plant_growth::PlantGrowth;
use crate::components::plant_type::PlantType;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct BasePlantBundle {
    type_: PlantType,
    growth: PlantGrowth,
}

impl BasePlantBundle {
    pub fn new(
        plant_type: PlantType,
        base_progress: f32,
        avg_growth_time_seconds: f32,
        avg_growth_step_size: f32,
        growth_step_randomness_range: f32,
    ) -> Self {
        let min_growth_step = avg_growth_step_size - growth_step_randomness_range;
        let max_growth_step = avg_growth_step_size + growth_step_randomness_range;
        let step_duration_seconds = avg_growth_time_seconds * avg_growth_step_size;

        Self {
            type_: plant_type,
            growth: PlantGrowth::new(
                base_progress,
                min_growth_step,
                max_growth_step,
                step_duration_seconds,
            ),
        }
    }
}