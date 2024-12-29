use crate::systems;
use bevy::app::App;
use bevy::prelude::{Plugin, Startup};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup, (
                systems::debug::spawn_grass_plant,
            ),
        );
    }
}