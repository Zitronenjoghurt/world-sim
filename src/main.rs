mod components;
mod bundles;
mod systems;
mod plugins;
mod enums;

use crate::plugins::map::MapPlugin;
use crate::plugins::plants::PlantsPlugin;
use crate::plugins::test::TestPlugin;
use bevy::prelude::{default, App, ImagePlugin, PluginGroup, Window, WindowPlugin};
use bevy::DefaultPlugins;

const MAP_HEIGHT: u32 = 20;
const MAP_WIDTH: u32 = 20;

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
        .add_plugins(TestPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(PlantsPlugin)
        .run();
}