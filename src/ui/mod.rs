pub mod button;
mod new_game_menu;
pub mod stat_display;
pub mod interaction_buttons;
pub mod play_area;
mod death_screen;
mod age_display;

pub use new_game_menu::*;
pub use death_screen::render_death_screen;
pub use age_display::draw_age_display;

use macroquad::color::BLACK;
use macroquad::prelude::{draw_text, measure_text};
use crate::game_state::GameState;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn draw_creature_name(state: &GameState) {
    let name_dimension = measure_text(state.creature().name(), None, 16, 1.0);
    draw_text(
        state.creature().name(),
        (SCREEN_WIDTH as f32 - name_dimension.width) / 2.0,
        (SCREEN_HEIGHT as f32 - name_dimension.height) / 2.0 + 7.0,
        16.0,
        BLACK
    );
}