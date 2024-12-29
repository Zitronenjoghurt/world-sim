use crate::components::nutrition::Nutrition;
use crate::components::plant_growth::PlantGrowth;
use crate::components::plant_type::PlantType;
use crate::enums::nutrition_type::NutritionType;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct BasePlantBundle {
    type_: PlantType,
    growth: PlantGrowth,
    nutrition: Nutrition,
}

impl BasePlantBundle {
    pub fn new(
        plant_type: PlantType,
        max_nutrition: f32,
        base_progress: f32,
        avg_growth_time_seconds: f32,
        avg_growth_step_size: f32,
        growth_step_randomness_range: f32,
    ) -> Self {
        let min_growth_step = avg_growth_step_size - growth_step_randomness_range;
        let max_growth_step = avg_growth_step_size + growth_step_randomness_range;
        let step_duration_seconds = avg_growth_time_seconds * avg_growth_step_size;

        let nutrition = Nutrition::new(NutritionType::Plant, 0.0);

        Self {
            type_: plant_type,
            growth: PlantGrowth::new(
                base_progress,
                max_nutrition,
                min_growth_step,
                max_growth_step,
                step_duration_seconds,
            ),
            nutrition,
        }
    }
}