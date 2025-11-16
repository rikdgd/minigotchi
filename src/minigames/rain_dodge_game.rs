use macroquad::prelude::*;
use crate::utils::Location;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub trait MiniGame {
    async fn run_game(&mut self);
    fn is_running(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct RainDodgeGame {
    player_x_pos: f32,
    drop_locations: Vec<Location>,
    health: i32,
    score: i32,
}

impl MiniGame for RainDodgeGame {
    async fn run_game(&mut self) {
        todo!()
    }

    fn is_running(&self) -> bool {
        self.health > 0
    }
}

impl RainDodgeGame {
    pub fn new() -> Self {
        Self {
            player_x_pos: (SCREEN_WIDTH / 2) as f32,
            drop_locations: Vec::new(),
            health: 100,
            score: 0,
        }
    }

    fn update_state(&mut self) {
        self.update_drop_locations();
    }

    fn update_drop_locations(&mut self) {
        let mut new_drops = Vec::new();

        // Update existing drop locations
        for drop in &mut self.drop_locations {
            drop.y -= 3.0;

            if drop.y > 0.0 {
                new_drops.push(drop.clone());
            }
        }

        // Generate new drops at the top of the screen
        let new_drop_count = rand::gen_range(0, 5);
        for _ in 0..new_drop_count {
            let new_drop = Location {
                x: rand::gen_range(0, SCREEN_WIDTH) as f32,
                y: rand::gen_range(0, SCREEN_HEIGHT) as f32,
            };

            new_drops.push(new_drop);
        }

        self.drop_locations = new_drops;
    }
}