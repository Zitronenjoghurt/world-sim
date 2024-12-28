use crate::systems;
use bevy::app::{App, Plugin};
use bevy::prelude::Update;

pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, (
                systems::plants::grow_plants,
                systems::plants::natural_grass_growth_tiles
            ),
        );
    }
}