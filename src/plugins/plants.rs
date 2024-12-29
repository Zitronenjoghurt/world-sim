use crate::systems;
use bevy::app::App;
use bevy::prelude::{Plugin, Update};

pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, (
                systems::plants::grow_plants,
                systems::plants::update_plant_visuals
            ),
        );
    }
}