use chrono::{DateTime, Utc};
use crate::plant::Plant;

#[derive(Debug, Clone)]
pub struct CompletedPlant {
    pub plant: Plant,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Garden {
    pub completed_plants: Vec<CompletedPlant>,
    pub current_streak: u32,
    pub longest_streak: u32,
}

impl Garden {
    pub fn new() -> Self {
        Garden {
            completed_plants: vec![],
            current_streak: 0,
            longest_streak: 0,
        }
    }

    pub fn add_completed_plant(&mut self, plant: Plant) {
        let completed = CompletedPlant {
            plant,
            completed_at: Utc::now(),
        };
        self.completed_plants.push(completed);
        self.current_streak += 1;
        if self.current_streak > self.longest_streak {
            self.longest_streak = self.current_streak;
        }
    }

    pub fn reset_streak(&mut self) {
        self.current_streak = 0;
    }

    pub fn total_completed(&self) -> usize {
        self.completed_plants.len()
    }
}