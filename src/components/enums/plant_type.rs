use bevy::prelude::Component;

#[derive(Component)]
#[repr(u8)]
pub enum PlantType {
    Grass = 0,
}