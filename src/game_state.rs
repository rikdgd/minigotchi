use macroquad::prelude::*;
use macroquad::file::load_file;
use macroquad::input::mouse_position;
use macroquad::rand::gen_range;
use crate::animations::Animation;
use crate::creature::{Creature, GrowthStage};
use crate::CREATURE_BASE_LOCATION;
use crate::movements::{CreatureMovement, CursorStalk, SicknessShakeMovement, get_creature_movement};
use crate::shapes::CreatureShapes;
use crate::save_management::{SaveState, store_save_state};
use crate::ui::play_area::{play_area_center, PLAY_AREA_RECT};
use crate::utils::{Location, time::get_now_millis};
use crate::items::inventory::Inventory;


pub struct GameState {
    creature: Creature,
    last_coin_time: i64,
    pub inventory: Inventory,
    pub prev_growth_stage: GrowthStage,
    pub creature_movement: Box<dyn CreatureMovement>,
    pub animation_queue: Vec<Box<dyn Animation>>,
    is_stalking_cursor: bool,
    sickness_movement_playing: bool,
}

impl GameState {
    pub fn new(creature_name: &str, creature_shape: CreatureShapes) -> Self {
        let now = get_now_millis();
        let creature = Creature::new(creature_name, creature_shape, now);
        let prev_growth_stage = creature.growth_stage();

        Self {
            creature_movement: get_creature_movement(&creature, CREATURE_BASE_LOCATION),
            creature,
            last_coin_time: now,
            inventory: Inventory::default(),
            prev_growth_stage,
            animation_queue: Vec::new(),
            is_stalking_cursor: false,
            sickness_movement_playing: false,
        }
    }
    
    pub async fn from_file(path: &str) -> Result<Self, macroquad::Error> {
        let file_bytes = load_file(path).await?;
        let content_string = String::from_utf8_lossy(&file_bytes);

        let state: SaveState = serde_json::from_str(&content_string)
            .expect("Failed to deserialize GameState from savefile");

        Ok(state.into())
    }

    pub fn update(&mut self) {
        let now = get_now_millis();

        // Update the game's state
        self.creature.update_state(now);
        self.update_coins(now);

        // Remove the first animation when it has finished playing
        if !self.animation_queue.is_empty() && !self.animation_queue[0].playing() {
            self.animation_queue.remove(0);
        }

        // Update the creature's movement if it happens to "evolve"
        if self.prev_growth_stage != self.creature().growth_stage() {
            self.creature_movement = get_creature_movement(
                self.creature(),
                CREATURE_BASE_LOCATION
            );
            self.prev_growth_stage = self.creature().growth_stage();
        }

        self.toggle_cursor_stalking();
        self.toggle_sickness_movement();
    }
    
    fn update_coins(&mut self, now: i64) {
        const DAY_MILLIS: i64 = 1000 * 60 * 60 * 24;
        
        while now - self.last_coin_time >= DAY_MILLIS {
            self.inventory.coins += 1;
            self.last_coin_time += DAY_MILLIS;
        }
    }
    
    pub fn creature(&self) -> &Creature {
        &self.creature
    }
    
    pub fn creature_mut(&mut self) -> &mut Creature {
        &mut self.creature
    }
    
    pub fn last_coin_time(&self) -> i64 {
        self.last_coin_time
    }

    /// Adds a new animation to the animation queue.
    pub fn push_animation<T: Animation + 'static>(&mut self, animation: T) {
        self.animation_queue.push(Box::new(animation));
    }

    fn should_follow_cursor(&self) -> bool {
        self.creature.growth_stage() == GrowthStage::Adult &&
        !self.creature.is_asleep() &&
        !self.creature.is_sick()
    }
    
    /// The **toggle_cursor_stalking** function toggles `self.is_stalking_cursor` when appropriate. 
    /// This in turn sets the creature's movement to `movements::cursor_stalk::CursorStalk`.
    fn toggle_cursor_stalking(&mut self) {
        if self.should_follow_cursor() {
            // Make the creature move towards the mouse pointer when it is in the playing area
            let mouse_pos: Vec2 = mouse_position().into();
            if PLAY_AREA_RECT.contains(mouse_pos) && !self.is_stalking_cursor {
                self.is_stalking_cursor = true;
                self.creature_movement = Box::new(CursorStalk::new(
                    self.creature_movement.next_location(),
                    self.creature()
                ));
            }

            // Stop the creature from moving towards the mouse pointer when it's not inside
            // the play area anymore
            if !PLAY_AREA_RECT.contains(mouse_pos) && self.is_stalking_cursor {
                self.is_stalking_cursor = false;
                let new_movement = get_creature_movement(
                    self.creature(),
                    self.creature_movement.current_location()
                );

                self.creature_movement = new_movement;
            }
        }
    }
    
    /// Sets the creature movement to the dedicated `SicknessShakeMovement` movement when it is sick,
    /// and disables it when the creature is cured.
    fn toggle_sickness_movement(&mut self) {
        if self.creature().is_sick() && !self.sickness_movement_playing {
            self.creature_movement = Box::new(SicknessShakeMovement::new(self.creature()));
            self.sickness_movement_playing = true;
        }
        
        if !self.creature.is_sick() && self.sickness_movement_playing {
            let mut center_location = play_area_center();
            center_location.x -= (self.creature().texture().width() / 2.0).round();
            center_location.y -= (self.creature().texture().height() / 2.0).round();
            
            self.creature_movement = get_creature_movement(self.creature(), center_location);
            self.sickness_movement_playing = false;
        }
    }
}

impl From<SaveState> for GameState {
    fn from(value: SaveState) -> Self {
        // When the game is freshly loaded from a file and the creature is adult, randomize the starting location
        // of its movement.
        let base_location = if value.creature.growth_stage() == GrowthStage::Adult {
            Location {
                x: gen_range(PLAY_AREA_RECT.left(), PLAY_AREA_RECT.right() - value.creature.texture().width()).round(),
                y: gen_range(PLAY_AREA_RECT.top(), PLAY_AREA_RECT.bottom() - value.creature.texture().height()).round(),
            }
        } else {
            CREATURE_BASE_LOCATION
        };
        
        Self {
            creature_movement: get_creature_movement(&value.creature, base_location),
            last_coin_time: value.last_coin_time,
            inventory: value.inventory,
            prev_growth_stage: value.creature.growth_stage(),
            animation_queue: Vec::new(),
            is_stalking_cursor: false,
            sickness_movement_playing: value.creature.is_sick(),
            creature: value.creature,
        }
    }
}

impl Drop for GameState {
    fn drop(&mut self) {
        let state = &(*self);
        store_save_state(state.into()).expect("Failed to save the game to disk");
    }
}


#[cfg(test)]
mod tests {
    use crate::game_state::GameState;
    use crate::shapes::CreatureShapes;
    
    const DAY_MILLIS: i64 = 1000 * 60 * 60 * 24;
    
    #[test]
    fn test_coin_updates() {
        let mut days_1_state = GameState::new("test", CreatureShapes::Sheep);
        let mut days_2_state = GameState::new("test", CreatureShapes::Sheep);
        let mut hours_13_state = GameState::new("test", CreatureShapes::Sheep);
        let mut days_100_state = GameState::new("test", CreatureShapes::Sheep);
        
        
        days_1_state.last_coin_time -= DAY_MILLIS;
        days_1_state.update();
        
        days_2_state.last_coin_time -= ((DAY_MILLIS * 2) as f32 * 1.1) as i64; // Overshoot by 10%
        days_2_state.update();
        
        hours_13_state.last_coin_time -= 1000 * 60 * 60 * 13;
        hours_13_state.update();
        
        days_100_state.last_coin_time -= DAY_MILLIS * 100;
        days_100_state.update();
        
        
        assert_eq!(1, days_1_state.inventory.coins);
        assert_eq!(2, days_2_state.inventory.coins);
        assert_eq!(0, hours_13_state.inventory.coins);
        assert_eq!(100, days_100_state.inventory.coins);
    }
}
