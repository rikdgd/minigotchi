use macroquad::prelude::*;
use macroquad::file::load_file;
use macroquad::input::mouse_position;
use crate::animations::Animation;
use crate::creature::{Creature, GrowthStage};
use crate::CREATURE_BASE_LOCATION;
use crate::movements::{get_creature_movement, CreatureMovement, CursorStalk, SicknessShakeMovement};
use crate::shapes::CreatureShapes;
use crate::save_management::store_game_state;
use crate::ui::play_area::{play_area_center, PLAY_AREA_RECT};
use crate::utils::time::get_now_millis;


pub struct GameState {
    creature: Creature,
    pub prev_growth_stage: GrowthStage,
    pub creature_movement: Box<dyn CreatureMovement>,
    pub current_animation: Option<Box<dyn Animation>>,
    is_stalking_cursor: bool,
    sickness_movement_playing: bool,
}

impl GameState {
    pub fn new(creature_name: &str) -> Self {
        let now = get_now_millis();
        let creature = Creature::new(creature_name, CreatureShapes::new_random(), now);
        let prev_growth_stage = creature.growth_stage();

        Self {
            creature_movement: get_creature_movement(&creature, CREATURE_BASE_LOCATION),
            creature,
            prev_growth_stage,
            current_animation: None,
            is_stalking_cursor: false,
            sickness_movement_playing: false,
        }
    }

    fn from_creature(creature: Creature) -> Self {
        Self {
            creature_movement: get_creature_movement(&creature, CREATURE_BASE_LOCATION),
            prev_growth_stage: creature.growth_stage(),
            current_animation: None,
            is_stalking_cursor: false,
            sickness_movement_playing: creature.is_sick(),
            creature,
        }
    }
    
    pub async fn from_file(path: &str) -> Result<Self, macroquad::Error> {
        let file_bytes = load_file(path).await?;
        let content_string = String::from_utf8_lossy(&file_bytes);

        let creature: Creature = serde_json::from_str(&content_string)
            .expect("Failed to deserialize GameState from savefile");

        Ok(Self::from_creature(creature))
    }

    pub fn update(&mut self) {
        let now = get_now_millis();

        // Update the creature's state
        self.creature.update_state(now);

        // Set the animation to None when it has finished
        if let Some(animation) = &self.current_animation {
            if !animation.playing() {
                self.current_animation = None;
            }
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
    
    pub fn creature(&self) -> &Creature {
        &self.creature
    }
    
    pub fn creature_mut(&mut self) -> &mut Creature {
        &mut self.creature
    }

    /// Sets the `current_animation` to a new animation, if it is already set this method does **nothing**.
    pub fn set_animation<T: Animation + 'static>(&mut self, animation: T) {
        if self.current_animation.is_none() {
            self.current_animation = Some(Box::new(animation));
        }
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
            self.creature_movement = get_creature_movement(self.creature(), play_area_center());
            self.sickness_movement_playing = false;
        }
    }
}

impl Drop for GameState {
    fn drop(&mut self) {
        store_game_state(self).expect("Failed to save the game to disk");
    }
}