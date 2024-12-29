use crate::components::growth_stages::GrowthStages;
use crate::components::nutrition::Nutrition;
use crate::components::plant_growth::PlantGrowth;
use bevy::prelude::{Changed, Query, Res, Sprite, Time};

pub fn grow_plants(
    time: Res<Time>,
    mut query: Query<&mut PlantGrowth>,
) {
    for mut plant_growth in query.iter_mut() {
        plant_growth.tick(time.delta());
    }
}

pub fn update_plant_visuals(
    mut query: Query<(&PlantGrowth, &mut Sprite, &GrowthStages), Changed<PlantGrowth>>
) {
    for (growth, mut sprite, stages) in query.iter_mut() {
        if let Some(texture_atlas) = &mut sprite.texture_atlas {
            let stage_index = growth.get_current_growth_stage_index(stages.count());
            texture_atlas.index = stages.get_index(stage_index);
        }
    }
}

pub fn update_plant_nutrition(
    mut query: Query<(&PlantGrowth, &mut Nutrition), Changed<PlantGrowth>>
) {
    for (growth, mut nutrition) in query.iter_mut() {
        let nutrition_value = growth.get_nutrition();
        nutrition.set_value(nutrition_value);
    }
}