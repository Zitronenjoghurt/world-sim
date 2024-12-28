use bevy::prelude::{Component, Deref, DerefMut, Timer, TimerMode};

#[derive(Component, Deref, DerefMut)]
pub struct GrowthTimer(Timer);

impl GrowthTimer {
    pub fn from_seconds(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Repeating))
    }
}