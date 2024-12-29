use crate::components::enums::plant_type::PlantType;
use crate::components::growth_timer::GrowthTimer;
use crate::components::nutrition::Nutrition;
use crate::enums::nutrition_type::NutritionType;
use bevy::prelude::{Commands, Component, Entity};
use std::time::Duration;

#[derive(Component)]
pub struct Plant {
    plant_type: PlantType,
    growth_timer: GrowthTimer,
    growth_stage: usize,
    max_growth_stage: usize,
    max_nutrition: f32,
    nutrition: Nutrition,
}

impl Plant {
    pub fn new(
        plant_type: PlantType,
        growth_stages_count: usize,
        growth_time: f32,
        max_nutrition: f32,
    ) -> Self {
        let nutrition_per_growth_stage = max_nutrition / growth_stages_count as f32;
        Self {
            plant_type,
            growth_timer: GrowthTimer::from_seconds(growth_time),
            growth_stage: 0,
            max_growth_stage: growth_stages_count.saturating_sub(1),
            max_nutrition,
            nutrition: Nutrition::new(
                nutrition_per_growth_stage,
                NutritionType::Plant,
                Plant::on_consume,
                Plant::on_consumed,
            ),
        }
    }

    pub fn get_growth_stage(&self) -> usize {
        self.growth_stage
    }

    pub fn get_nutrition_per_growth_stage(&self) -> f32 {
        self.max_nutrition / (self.max_growth_stage + 1) as f32
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
            self.nutrition.add_value(self.get_nutrition_per_growth_stage());
            true
        } else {
            false
        }
    }

    pub fn on_consume(_entity: Entity, _amount: f32, _commands: &mut Commands) {}

    pub fn on_consumed(entity: Entity, commands: &mut Commands) {
        println!("Owie I was eaten! {}", entity);
        commands.entity(entity).despawn();
    }
}