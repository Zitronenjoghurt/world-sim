use bevy::prelude::Component;

#[derive(Component)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }

    pub fn set(&mut self, name: &str) {
        self.0 = name.to_string();
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}