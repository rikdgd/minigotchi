use macroquad::file::load_file;
use serde::{Serialize, Deserialize};
use crate::creature::{Creature, GrowthStage};
use crate::shapes::CreatureShapes;
use crate::save_management::store_game_state;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    creature: Creature,
    pub prev_growth_stage: GrowthStage,
    last_update_time: i64,
}

impl GameState {
    pub fn new(creature_name: &str) -> Self {
        let creature = Creature::new(creature_name, CreatureShapes::new_random());
        let prev_growth_stage = creature.growth_stage();

        Self {
            creature,
            prev_growth_stage,
            last_update_time: chrono::Utc::now().timestamp_millis(),
        }
    }
    
    pub async fn from_file(path: &str) -> Result<Self, macroquad::Error> {
        let file_bytes = load_file(path).await?;
        let content_string = String::from_utf8_lossy(&file_bytes);

        let state: Self = serde_json::from_str(&content_string)
            .expect("Failed to deserialize GameState from savefile");

        Ok(state)
    }

    pub fn update(&mut self) {
        let now = chrono::Utc::now().timestamp_millis();
        self.last_update_time = now;
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