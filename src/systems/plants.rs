use crate::components::growth_stages::GrowthStages;
use crate::components::plant::Plant;
use bevy::prelude::{Query, Res, Sprite, Time};

pub fn grow_plants(
    mut query: Query<(
        &mut Plant,
        Option<(&mut Sprite, &GrowthStages)>
    )>,
    time: Res<Time>,
) {
    for (mut plant, sprite_data) in query.iter_mut() {
        let new_growth_stage_reached = plant.tick_growth(time.delta());

        if new_growth_stage_reached {
            if let Some((mut sprite, growth_stages)) = sprite_data {
                if let Some(texture_atlas) = &mut sprite.texture_atlas {
                    texture_atlas.index = growth_stages.get_index(plant.get_growth_stage())
                }
            }
        }
    }
}