use crate::enums::nutrition_type::NutritionType;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Nutrition {
    type_: NutritionType,
    value: f32,
}

impl Nutrition {
    pub fn new(nutrition_type: NutritionType, value: f32) -> Self {
        Self {
            type_: nutrition_type,
            value,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    pub fn get_type(&self) -> NutritionType {
        self.type_
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }
}