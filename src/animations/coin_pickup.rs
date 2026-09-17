use macroquad::prelude::*;
use crate::{animations::{Animation, PopupAnimation}, include_texture};


const FRAME_COUNT: usize = 2;

#[derive(Debug, Clone)]
pub struct CoinPickupAnimation {
    frames: [Texture2D; FRAME_COUNT],
    frame_index: usize,
    frame_timer: f32,
    playing: bool,
}

impl CoinPickupAnimation {
    fn update(&mut self) {
        if self.frame_index > 4 {
            self.playing = false;
            return;
        }
        
        self.frame_timer += get_frame_time();
        if self.frame_timer > 0.5 {
            self.frame_timer = 0.;
            self.frame_index += 1;
        }
    }
}

impl Animation for CoinPickupAnimation {
    fn render(&mut self) {
        self.draw_background();
        
        let draw_loc = self.frame_draw_location();
        draw_texture_ex(
            &self.frames[self.frame_index % FRAME_COUNT],
            draw_loc.x,
            draw_loc.y,
            BLACK,
            DrawTextureParams::default(),
        );
        
        self.update();
    }

    fn dimensions(&self) -> crate::utils::Dimensions {
        (
            self.frames[0].width(),
            self.frames[0].height(),
        ).into()
    }

    fn playing(&self) -> bool {
        self.playing
    }
}

impl PopupAnimation for CoinPickupAnimation {}

impl Default for CoinPickupAnimation {
    fn default() -> Self {
        Self { 
            // TODO: Create actual frames for the coin animation
            frames: [
                include_texture!("../../resources/animations/coin/coin0.png"),
                include_texture!("../../resources/animations/coin/coin1.png"),
            ],
            frame_index: Default::default(),
            frame_timer: Default::default(),
            playing: true,
        }
    }
}