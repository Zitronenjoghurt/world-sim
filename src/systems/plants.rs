use crate::bundles::plants::grass::GrassPlantBundle;
use crate::components::growth_stages::GrowthStages;
use crate::components::natural_grass_growth::NaturalGrassGrowth;
use crate::components::plant::Plant;
use crate::helpers::coordinates::get_random_coordinates_from_tile_pos;
use bevy::asset::{AssetServer, Assets};
use bevy::prelude::{Commands, Query, Res, ResMut, Sprite, TextureAtlasLayout, Time, Vec2};
use bevy_ecs_tilemap::prelude::TilePos;

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

pub fn natural_grass_growth_tiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut NaturalGrassGrowth, &TilePos)>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (mut grass_growth, tile_pos) in query.iter_mut() {
        let grass_grown = grass_growth.tick(time.delta());
        if grass_grown {
            let (x, y) = get_random_coordinates_from_tile_pos(tile_pos, 3.0);

            commands.spawn(GrassPlantBundle::new(
                &asset_server,
                &mut texture_atlas_layouts,
                Vec2::new(x, y),
            ));
        }
    }
}