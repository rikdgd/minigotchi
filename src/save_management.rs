use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::Write;
use crate::game_state::GameState;

pub fn store_game_state(state: &GameState) -> std::io::Result<()> {
    let file_path = get_save_file_path();

    let game_state = serde_json::to_string_pretty(&state)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;


    file.write_all(game_state.as_bytes())?;
    file.flush()?;

    Ok(())
}

pub fn get_save_file_path() -> String {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().expect("Executable must be in some directory");
    let data_file_path: PathBuf = exe_dir.join("save-file.txt");

    data_file_path.to_str().unwrap().to_string()
}