use crate::systems;
use bevy::app::{App, Startup};
use bevy::prelude::{IntoSystemConfigs, Plugin};
use bevy_ecs_tilemap::TilemapPlugin;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);
        app.add_systems(
            Startup, (
                systems::map::spawn_camera,
                systems::map::spawn_maps,
                systems::map::spawn_tiles
            ).chain(),
        );
    }
}