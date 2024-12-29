mod helpers;
mod plugins;
mod components;
mod bundles;
mod systems;
mod enums;

use crate::plugins::debug::DebugPlugin;
use crate::plugins::map::MapPlugin;
use crate::plugins::plants::PlantsPlugin;
use bevy::prelude::{default, App, ImagePlugin, PluginGroup, Window, WindowPlugin};
use bevy::DefaultPlugins;

const TILE_SIZE: u32 = 8;
const MAP_HEIGHT_TILES: u32 = 20;
const MAP_WIDTH_TILES: u32 = 20;
const MAP_HEIGHT_PX: u32 = TILE_SIZE * MAP_HEIGHT_TILES;
const MAP_WIDTH_PX: u32 = MAP_WIDTH_TILES * TILE_SIZE;
const HALF_MAP_HEIGHT_PX: u32 = MAP_HEIGHT_PX / 2;
const HALF_MAP_WIDTH_PX: u32 = MAP_WIDTH_PX / 2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(
                        Window {
                            title: "World Sim".to_string(),
                            ..default()
                        }
                    ),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(MapPlugin)
        .add_plugins(PlantsPlugin)
        .add_plugins(DebugPlugin)
        .run();
}