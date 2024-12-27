use bevy::prelude::{Commands, Query, With};
use crate::bundles::rabbit::RabbitBundle;
use crate::components::generic_properties::Name;
use crate::components::tags::Rabbit;

pub fn spawn_rabbits(mut commands: Commands) {
    commands.spawn(RabbitBundle::default());
    commands.spawn(RabbitBundle::new("Cookie"));
}

pub fn update_rabbit_names(mut query: Query<&mut Name, With<Rabbit>>) {
    for mut name in &mut query {
        if name.as_str() == "Cookie" {
            name.set("Cookiedookie");
        }
    }
}

pub fn greet_rabbits(query: Query<&Name, With<Rabbit>>) {
    for name in &query {
        println!("Oh hello lil bunny '{}'!", name.as_str());
    }
}