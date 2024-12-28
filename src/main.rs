mod components;
mod bundles;
mod systems;
mod plugins;
mod enums;

use crate::plugins::map::MapPlugin;
use bevy::prelude::{default, App, ImagePlugin, PluginGroup, Window, WindowPlugin};
use bevy::DefaultPlugins;

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
        //.add_plugins(TestPlugin)
        .add_plugins(MapPlugin)
        .run();
}