use crate::bundles::plants::grass::GrassPlantBundle;
use bevy::prelude::{AssetServer, Assets, Commands, Res, ResMut, TextureAtlasLayout};

pub fn spawn_grass_plant(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let plant_bundle = GrassPlantBundle::new(
        &asset_server,
        &mut texture_atlas_layouts,
        0.0,
        0.0,
    );
    commands.spawn(plant_bundle);
}