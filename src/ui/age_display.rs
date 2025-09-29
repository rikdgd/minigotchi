use macroquad::prelude::*;
use crate::{GameState, SCREEN_WIDTH};
use crate::utils::time::get_now_millis;

pub fn draw_age_display(state: &GameState) {
    let now_millis = get_now_millis().unwrap();
    let created_millis = state.creature().time_created();
    let hours_alive = (now_millis - created_millis) / 1000 / 60 / 60;

    let text = format!(
        "age: {} days and {} hours",
        hours_alive / 24,
        hours_alive % 24,
    );
    let text_dim = measure_text(&text, None, 10, 1.0);

    let x = SCREEN_WIDTH as f32 / 2.0 - text_dim.width / 2.0;
    draw_text(&text, x, text_dim.height, 10.0, BLACK);
}