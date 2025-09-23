use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::ui::button::Button;
use crate::BACKGROUND_COLOR;

pub fn render_death_screen(state: &GameState) {
    let confirm_btn = Button {
        pos: Vec2::new(75.0, 150.0),
        text: String::from(":("),
        ..Default::default()
    };

    loop {
        clear_background(BACKGROUND_COLOR);
        
        if confirm_btn.is_clicked() {
            break;
        }
    }
}