use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct GrowthStages(pub Vec<usize>);

impl GrowthStages {
    pub fn get_initial_index(&self) -> usize {
        self.0[0]
    }

    pub fn get_index(&self, index: usize) -> usize {
        self.0[index]
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
}