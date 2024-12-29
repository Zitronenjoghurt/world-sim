use bevy::prelude::{Component, Timer, TimerMode};
use rand::Rng;
use std::time::Duration;

#[derive(Component)]
pub struct PlantGrowth {
    progress: f32,
    min_progress_per_step: f32,
    max_progress_per_step: f32,
    timer: Timer,
}

impl PlantGrowth {
    pub fn new(
        base_progress: f32,
        min_progress_per_step: f32,
        max_progress_per_step: f32,
        step_duration_seconds: f32,
    ) -> Self {
        Self {
            progress: base_progress,
            min_progress_per_step,
            max_progress_per_step,
            timer: Timer::from_seconds(step_duration_seconds, TimerMode::Repeating),
        }
    }

    pub fn is_growth_finished(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn tick(&mut self, duration: Duration) {
        if self.is_growth_finished() {
            return;
        }

        self.timer.tick(duration);
        if self.timer.just_finished() {
            let mut rng = rand::thread_rng();
            let step_progress = rng.gen_range(self.min_progress_per_step..=self.max_progress_per_step);
            self.progress += step_progress;
            self.progress = self.progress.clamp(0.0, 1.0);
        }
    }

    pub fn get_current_growth_stage_index(&self, max_stages: usize) -> usize {
        (self.progress * (max_stages - 1) as f32).ceil() as usize
    }
}