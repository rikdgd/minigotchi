use macroquad::prelude::*;
use super::{Animation, PopupAnimation};
use crate::{include_texture, SCREEN_HEIGHT, SCREEN_WIDTH};


pub struct CreatureEatAnimation {
    current_frame: usize,
    frame_timer: f32,
}

impl Animation for CreatureEatAnimation {
    async fn render(&mut self) {
        let draw_location = self.frame_draw_location();
        
        loop {
            if self.frame_timer > 0.5 {
                self.current_frame += 1;
                self.frame_timer = 0.0;
            }
            
            draw_texture(
                &Self::frames()[self.current_frame],
                draw_location.x,
                draw_location.y,
                BLACK,
            )
        }

        todo!()
    }

    fn dimensions(&self) -> [f32; 2] {
        [15.0, 15.0]
    }
}

impl PopupAnimation for CreatureEatAnimation{}

impl CreatureEatAnimation {
    pub fn new() -> Self {
        Self {
            current_frame: 0,
            frame_timer: 0.0,
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