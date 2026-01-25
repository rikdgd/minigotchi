use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::ui::{new_game_menu::render_new_game_menu, button::Button};
use crate::{BACKGROUND_COLOR, SCREEN_WIDTH, SCREEN_HEIGHT};

const FONT_SIZE: u16 = 16;
const ROW_HEIGHT: f32 = 12.0;
const TEXT_ROW_COUNT: usize = 3;

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

    let death_texts: [String; TEXT_ROW_COUNT] = get_death_text(old_state);
    let text_dimensions: [TextDimensions; TEXT_ROW_COUNT] = [
        measure_text(&death_texts[0], None, FONT_SIZE + 2, 1.0),
        measure_text(&death_texts[1], None, FONT_SIZE, 1.0),
        measure_text(&death_texts[2], None, FONT_SIZE, 1.0),
    ];

    loop {
        clear_background(BACKGROUND_COLOR);

        if confirm_btn.is_clicked() {
            break;
        }

        confirm_btn.render();

        for i in 0..TEXT_ROW_COUNT {
            let font_size = if i == 0 {
                FONT_SIZE + 5
            } else {
                FONT_SIZE
            };

            draw_text(
                &death_texts[i],
                (SCREEN_WIDTH as f32 - text_dimensions[i].width) / 2.0,
                (SCREEN_HEIGHT as f32 - text_dimensions[i].height) / 3.0 + ROW_HEIGHT * i as f32,
                font_size as f32,
                GRAY,
            );
        }

        next_frame().await;
    }

    // Also awaiting the next frame here ensures the new game menu is properly rendered.
    next_frame().await;
    render_new_game_menu().await
}

fn get_death_text(state: &GameState) -> [String; 3] {
    let death_time = state.creature().time_of_death()
        .expect("The creature has no recorded time of death");
    let millis_alive = death_time - state.creature().time_created();
    let hours_alive = millis_alive / 1000 / 60 / 60;

    [
        String::from(state.creature().name()),
        String::from("has died at the age of:"),
        format!(
            "{} days and {} hours",
            hours_alive / 24,
            hours_alive % 24,
        ),
    ]
}