use crate::enums::nutrition_type::NutritionType;
use bevy::audio::Sample;
use bevy::prelude::{Commands, Component, Entity};

#[derive(Component)]
pub struct Nutrition {
    value: f32,
    nutrition_type: NutritionType,
    on_consume: fn(Entity, f32, &mut Commands),
    on_consumed: fn(Entity, &mut Commands),
}

impl Nutrition {
    pub fn new(
        value: f32,
        nutrition_type: NutritionType,
        on_consume: fn(Entity, f32, &mut Commands),
        on_consumed: fn(Entity, &mut Commands),
    ) -> Self {
        Self { value, nutrition_type, on_consume, on_consumed }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    pub fn add_value(&mut self, value: f32) {
        self.value = self.value.saturating_add(value);
    }

    pub fn consume(&mut self, amount: f32, entity: Entity, commands: &mut Commands) -> f32 {
        let consumed = amount.min(self.value);
        self.value -= consumed;
        (self.on_consume)(entity, consumed, commands);
        if self.value <= 0.0 {
            (self.on_consumed)(entity, commands);
        }
        consumed
    }

    pub fn consume_all(&mut self, entity: Entity, commands: &mut Commands) -> f32 {
        self.consume(self.value, entity, commands)
    }

    pub fn is_consumed(&self) -> bool {
        self.value <= 0.0
    }
}