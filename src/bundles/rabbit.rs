use crate::components::generic_properties::name::Name;
use crate::components::generic_properties::position::Position;
use crate::components::tags::Rabbit;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct RabbitBundle {
    creature: Rabbit,
    name: Name,
    position: Position,
}

impl Default for RabbitBundle {
    fn default() -> Self {
        Self {
            creature: Rabbit,
            name: Name::new("Rabbit"),
            position: Position::default(),
        }
    }
}

impl RabbitBundle {
    pub fn new(name: &str, x: i32, y: i32) -> Self {
        RabbitBundle {
            creature: Rabbit,
            name: Name::new(name),
            position: Position::new(x, y),
        }
    }
}