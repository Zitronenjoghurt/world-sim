use bevy::prelude::Component;

#[derive(Component)]
pub struct GrowthStages(pub Vec<usize>);

impl GrowthStages {
    pub fn get_index(&self, growth_stage: usize) -> usize {
        self.0[growth_stage]
    }
}