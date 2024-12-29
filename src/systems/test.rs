use crate::bundles::plants::grass::GrassPlantBundle;
use crate::bundles::rabbit::RabbitBundle;
use crate::components::generic_properties::name::Name;
use crate::components::nutrition::Nutrition;
use crate::components::tags::Rabbit;
use bevy::prelude::{AssetServer, Assets, Commands, Entity, Query, Res, ResMut, TextureAtlasLayout, Vec2, With};

pub fn spawn_rabbits(mut commands: Commands) {
    commands.spawn(RabbitBundle::default());
    commands.spawn(RabbitBundle::new("Cookie", 0, 0));
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

pub fn spawn_grass(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(
        GrassPlantBundle::new(
            &asset_server,
            &mut texture_atlas_layouts,
            Vec2::new(0.0, 0.0),
        )
    );
}

pub fn consume_stuff(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Nutrition)>,
) {
    println!("Consuming stuff!");
    for (entity, mut nutrition) in query.iter_mut() {
        println!("Yep");
        nutrition.consume_all(entity, &mut commands);
    }
}