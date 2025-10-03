use macroquad::prelude::*;
use super::{Animation, PopupAnimation};
use crate::include_texture;


/// A `PopupAnimation` that shows the creature eating some food.
pub struct CreatureEatAnimation {
    current_frame: usize,
    frame_timer: f32,
    playing: bool,
}

impl Animation for CreatureEatAnimation {
    fn render(&mut self) {
        let draw_location = self.frame_draw_location();

        self.draw_background();
        draw_texture(
            &Self::frames()[self.current_frame],
            draw_location.x,
            draw_location.y,
            BLACK,
        );

        self.update_state();
    }

    fn dimensions(&self) -> [f32; 2] {
        [15.0, 15.0]
    }

    fn playing(&self) -> bool {
        self.playing
    }
}

impl PopupAnimation for CreatureEatAnimation{}

impl CreatureEatAnimation {
    pub fn new() -> Self {
        Self {
            current_frame: 0,
            frame_timer: 0.0,
            playing: true,
        }
    }

    fn update_state(&mut self) {
        if self.frame_timer > 0.75 {
            self.current_frame += 1;
            self.frame_timer = 0.0;
        }

        self.frame_timer += get_frame_time();

        if self.current_frame >= Self::frames().len() {
            self.playing = false;
        }
    }

    fn frames() -> [Texture2D; 4] {
        [
            include_texture!("../../resources/animations/eating/burger0.png"),
            include_texture!("../../resources/animations/eating/burger1.png"),
            include_texture!("../../resources/animations/eating/burger2.png"),
            include_texture!("../../resources/animations/eating/burger3.png"),
        ]
    }
}
