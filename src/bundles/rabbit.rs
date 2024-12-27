use bevy::prelude::Bundle;
use crate::components::generic_properties::Name;
use crate::components::tags::Rabbit;

#[derive(Bundle)]
pub struct RabbitBundle {
    creature: Rabbit,
    name: Name,
}

impl Default for RabbitBundle {
    fn default() -> Self {
        Self {
            creature: Rabbit,
            name: Name::new("Rabbit"),
        }
    }
}

impl RabbitBundle {
    pub fn new(name: &str) -> Self {
        RabbitBundle {
            creature: Rabbit,
            name: Name::new(name),
        }
    }
}