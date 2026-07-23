use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::Write;
use serde::{Deserialize, Serialize};

use crate::game_state::GameState;
use crate::creature::Creature;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveState {
    pub creature: Creature,
    pub coins: u32,
}

impl From<&GameState> for SaveState {
    fn from(value: &GameState) -> Self {
        Self {
            creature: value.creature().clone(),
            coins: value.coins(),
        }
    }
}

pub fn store_game_state(state: SaveState) -> std::io::Result<()> {
    let save = serde_json::to_string_pretty(&state)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(get_save_file_path())?;


    file.write_all(save.as_bytes())?;
    file.flush()?;

    Ok(())
}

pub fn get_save_file_path() -> String {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().expect("Executable must be in some directory");
    let data_file_path: PathBuf = exe_dir.join("save-file.txt");

    data_file_path.to_str().unwrap().to_string()
}