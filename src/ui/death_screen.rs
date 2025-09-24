use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::ui::{new_game_menu::render_new_game_menu, button::Button};
use crate::{BACKGROUND_COLOR, SCREEN_WIDTH, SCREEN_HEIGHT};
use chrono::Utc;

const FONT_SIZE: u16 = 16;

pub async fn render_death_screen(old_state: &GameState) -> GameState {
    let confirm_btn = Button {
        pos: Vec2::new(
            (SCREEN_WIDTH as f32 - 100.0) / 2.0,
            150.0
        ),
        text: String::from("new creature"),
        size: Vec2::new(100.0, 20.0),
        ..Default::default()
    };

    let text_size_1 = measure_text(&get_death_text(old_state)[0], None, FONT_SIZE, 1.0);
    let text_size_2 = measure_text(&get_death_text(old_state)[1], None, FONT_SIZE, 1.0);

    loop {
        clear_background(BACKGROUND_COLOR);

        if confirm_btn.is_clicked() {
            break;
        }

        confirm_btn.render();

        draw_text(
            &get_death_text(old_state)[0],
            (SCREEN_WIDTH as f32 - text_size_1.width) / 2.0,
            (SCREEN_HEIGHT as f32 - text_size_1.height) / 3.0,
            FONT_SIZE as f32,
            GRAY,
        );
        draw_text(
            &get_death_text(old_state)[1],
            (SCREEN_WIDTH as f32 - text_size_2.width) / 2.0,
            (SCREEN_HEIGHT as f32 - text_size_2.height) / 3.0 + text_size_2.height,
            FONT_SIZE as f32,
            GRAY,
        );

        next_frame().await;
    }

    // Also awaiting the next frame here ensures the new game menu is properly rendered.
    next_frame().await;
    render_new_game_menu().await
}

fn get_death_text(state: &GameState) -> [String; 2] {
    let now = Utc::now().timestamp_millis();
    let millis_alive = now - state.friend().time_created();
    let hours_alive = millis_alive / 1000 / 60 / 60;

    [
        format!(
            "{} has died at the age of",
            state.friend().name(),
        ),
        format!(
            "{} days and {} hours",
            hours_alive / 24,
            hours_alive % 24,
        ),
    ]
}