use macroquad::color::BLACK;
use macroquad::prelude::{draw_texture, Texture2D};
use crate::include_texture;
use super::{Animation, PopupAnimation};

#[derive(Debug, Copy, Clone)]
pub struct SyringeAnimation {
    current_frame: usize,
    frame_timer: f32,
    playing: bool,
}

impl SyringeAnimation {
    fn frames() -> [Texture2D; 4] {
        [
            include_texture!("../../resources/animations/health/syringe0.png"),
            include_texture!("../../resources/animations/health/syringe1.png"),
            include_texture!("../../resources/animations/health/syringe2.png"),
            include_texture!("../../resources/animations/health/syringe3.png"),
        ]
    }

    fn update_state(&mut self) {

    }
}

impl Animation for SyringeAnimation {
    fn render(&mut self) {
        let draw_location = self.frame_draw_location();

        self.draw_background();

        // TODO: Draw the actual frames now
        draw_texture(
            &Self::frames()[0],
            draw_location.x,
            draw_location.y,
            BLACK
        );
    }

    fn dimensions(&self) -> [f32; 2] {
        [15.0, 15.0]
    }

    fn playing(&self) -> bool {
        self.playing
    }
}

impl PopupAnimation for SyringeAnimation {}