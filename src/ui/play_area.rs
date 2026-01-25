use macroquad::prelude::*;
use crate::creature::Creature;

pub const PLAY_AREA_RECT: Rect = Rect::new(10.0, 10.0, 180.0, 80.0);

pub fn draw_play_area(creature: &Creature) {
    let color = if creature.is_asleep() {
        Color::new(0.35, 0.35, 0.35, 1.0)
    } else {
        Color::new(0.7, 0.7, 0.7, 1.0)
    };

    draw_rectangle(
        PLAY_AREA_RECT.x,
        PLAY_AREA_RECT.y,
        PLAY_AREA_RECT.w,
        PLAY_AREA_RECT.h,
        color
    );
}
