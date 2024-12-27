use bevy::prelude::IntoSystemConfigs;
use bevy::app::{App, Startup};
use bevy::prelude::{Plugin, Update};
use crate::systems;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup, systems::test::spawn_rabbits
        );
        app.add_systems(
            Update, 
            (
                systems::test::update_rabbit_names, 
                systems::test::greet_rabbits
            ).chain()
        );
    }
}