use macroquad::prelude::*;
use crate::friend::Friend;

pub fn draw_play_area(friend: &Friend) {
    let color = if friend.is_asleep() {
        Color::new(0.35, 0.35, 0.35, 1.0)
    } else {
        Color::new(0.7, 0.7, 0.7, 1.0)
    };

    draw_rectangle(10.0, 10.0, 180.0, 80.0, color);
}