use macroquad::prelude::*;
use crate::creature::Creature;

pub fn draw_play_area(creature: &Creature) {
    let color = if creature.is_asleep() {
        Color::new(0.35, 0.35, 0.35, 1.0)
    } else {
        Color::new(0.7, 0.7, 0.7, 1.0)
    };

    draw_rectangle(10.0, 10.0, 180.0, 80.0, color);
}