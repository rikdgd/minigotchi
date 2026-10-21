use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::Write;
use serde::{Deserialize, Serialize};

use crate::game_state::GameState;
use crate::creature::Creature;
use crate::items::inventory::Inventory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveState {
    pub creature: Creature,
    pub last_coin_time: i64,
    pub inventory: Inventory,
}

impl From<&GameState> for SaveState {
    fn from(value: &GameState) -> Self {
        Self {
            creature: value.creature().clone(),
            last_coin_time: value.last_coin_time(),
            inventory: value.inventory.clone(),
        }
    }
}

pub fn store_save_state(state: SaveState) -> std::io::Result<()> {
    let save = serde_json::to_string_pretty(&state)?;
    let save_bytes = save.as_bytes();
    
    #[cfg(not(debug_assertions))]
    let save_bytes = &binary_invert_bytes(&save_bytes);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(get_save_file_path())?;


    file.write_all(save_bytes)?;
    file.flush()?;

    Ok(())
}

/// Retrieves the `SaveState` from the save file by reading it and then deserializing it.
pub async fn retrieve_save_state() -> Result<SaveState, macroquad::Error> {
    let path = get_save_file_path();
    let file_bytes = macroquad::file::load_file(&path).await?;
    
    #[cfg(not(debug_assertions))]
    let file_bytes = binary_invert_bytes(&file_bytes);
    
    let content_string = String::from_utf8_lossy(&file_bytes);

    let state: SaveState = serde_json::from_str(&content_string)
        .expect("Failed to deserialize GameState from savefile");
    
    Ok(state)
}

pub fn get_save_file_path() -> String {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().expect("Executable must be in some directory");
    let data_file_path: PathBuf = exe_dir.join("minigotchi.sav");

    data_file_path.to_str().unwrap().to_string()
}

#[cfg(not(debug_assertions))]
fn binary_invert_bytes(input: &[u8]) -> Vec<u8> {
    let mut inverted = Vec::new();
    
    for b in input {
        inverted.push(b ^ u8::MAX);
    }
    
    inverted
}


#[cfg(test)]
mod tests {
    use crate::game_state::GameState;
    use crate::save_management::SaveState;
    use crate::shapes::CreatureShape;

    #[test]
    fn game_save_state_conversion() {
        let original_state = GameState::new("test", CreatureShape::Sheep);
        let save_state: SaveState = (&original_state).into();
        
        let retrieved_state: GameState = save_state.into();
        
        assert_eq!(original_state.creature(), retrieved_state.creature());
        assert_eq!(original_state.last_coin_time(), retrieved_state.last_coin_time());
        assert_eq!(original_state.inventory, retrieved_state.inventory);
    }
}
