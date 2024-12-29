use crate::systems;
use bevy::app::App;
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{Plugin, Startup, Update};
use bevy::render::diagnostic::RenderDiagnosticsPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(EntityCountDiagnosticsPlugin);
        app.add_plugins(RenderDiagnosticsPlugin);
        app.add_plugins(LogDiagnosticsPlugin::default());
        app.add_systems(
            Startup, (
                systems::debug::spawn_grass_plant,
            ),
        );
        app.add_systems(
            Update,
            systems::debug::spawn_grass_on_click,
        );
    }
}