use macroquad::prelude::*;

pub fn draw_play_area(dark_mode: bool) {
    let color = if dark_mode {
        Color::new(0.35, 0.35, 0.35, 1.0)
    } else {
        Color::new(0.7, 0.7, 0.7, 1.0)
    };

    draw_rectangle(10.0, 10.0, 180.0, 80.0, color);
}