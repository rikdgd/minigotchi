use macroquad::prelude::*;
use crate::utils::Location;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};


const MAX_NEW_DROP_SPAWNS: i32 = 5;


/// Any implementation of this is a playable minigame inside minigotchi.
///
/// ## Methods:
/// * `run_game` - Calling this method runs and renders the minigame onto the screen.
/// * `is_running` - Returns a `bool` that tells whether the game has ended or not,
/// **true** if it is still running.
pub trait MiniGame {
    async fn run_game(&mut self);
}

/// A minigame in which the player needs to dodge falling droplets of rain.
#[derive(Debug, Clone)]
pub struct RainDodgeGame {
    player_x_pos: f32,
    drop_locations: Vec<Location>,
    health: i32,
    score: i32,
    update_timer: f32,
}

impl MiniGame for RainDodgeGame {
    async fn run_game(&mut self) {
        while self.health > 0 {
            self.update_timer += get_frame_time();
            if self.update_timer >= 1.0 {
                self.update_timer = 0.0;

                self.update_state();

                // TODO: Draw the raindrops
            }
        }
    }
}

impl RainDodgeGame {
    pub fn new() -> Self {
        Self {
            player_x_pos: (SCREEN_WIDTH / 2) as f32,
            drop_locations: Vec::new(),
            health: 100,
            score: 0,
            update_timer: 0.0,
        }
    }

    fn update_state(&mut self) {
        self.update_drop_locations();
        self.generate_new_drops();
    }

    /// Updates the drops in the current game's state. It makes existing drop locations move down,
    /// and adds up to **5** new droplets to the state. Drop locations that exit the screen are also
    /// removed.
    fn update_drop_locations(&mut self) {
        let mut new_drops = Vec::new();

        // Update existing drop locations
        for drop in &mut self.drop_locations {
            drop.y -= 3.0;

            if drop.y > 0.0 {
                new_drops.push(drop.clone());
            }
        }

        self.drop_locations = new_drops;
    }
    
    /// Generates some new raindrops at the top of the screen with a random x location. These are
    /// directly added to `self.drop_locations`.
    fn generate_new_drops(&mut self) {
        let new_drop_count = rand::gen_range(0, MAX_NEW_DROP_SPAWNS);
        for _ in 0..new_drop_count {
            let new_drop = Location {
                x: rand::gen_range(0, SCREEN_WIDTH) as f32,
                y: SCREEN_HEIGHT as f32,
            };

            self.drop_locations.push(new_drop);
        }
    }
}



#[cfg(test)]
mod Tests {
    use crate::utils::Location;
    use crate::minigames::rain_dodge_game::*;
    
    #[test]
    fn update_drop_locations() {
        let mut game = RainDodgeGame::new();
        game.drop_locations.push(Location { x: 0.0, y: 10.0 });
    }
}
