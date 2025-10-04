use macroquad::file::load_file;
use crate::animations::Animation;
use crate::creature::{Creature, GrowthStage};
use crate::CREATURE_BASE_LOCATION;
use crate::movements::{get_creature_movement, CreatureMovement};
use crate::shapes::CreatureShapes;
use crate::save_management::store_game_state;
use crate::utils::time::get_now_millis;


pub struct GameState {
    creature: Creature,
    pub prev_growth_stage: GrowthStage,
    creature_movement: Box<dyn CreatureMovement>,
    current_animation: Option<Box<dyn Animation>>,
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
        }
    }

    fn from_creature(creature: Creature) -> Self {
        Self {
            creature_movement: get_creature_movement(&creature, CREATURE_BASE_LOCATION),
            prev_growth_stage: creature.growth_stage(),
            creature,
            current_animation: None,
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
        self.creature.update_state(now);
    }
    
    pub fn creature(&self) -> &Creature {
        &self.creature
    }
    
    pub fn creature_mut(&mut self) -> &mut Creature {
        &mut self.creature
    }
}

impl Drop for GameState {
    fn drop(&mut self) {
        store_game_state(self).expect("Failed to save the game to disk");
    }
}