use crate::systems;
use bevy::app::{App, Startup};
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::Plugin;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup, (
                (
                    systems::test::spawn_rabbits,
                    systems::test::update_rabbit_names,
                    systems::test::greet_rabbits
                ).chain(),
                systems::test::spawn_grass
            ),
        );
    }
}