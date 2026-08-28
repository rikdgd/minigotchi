use macroquad::prelude::*;
use crate::animations::{Animation, PopupAnimation};
use crate::include_texture;
use crate::utils::Dimensions;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmotionAnimationType {
    Love,
    Sad,
    Tired,
}

#[derive(Debug, Clone)]
pub struct EmotionAnimation {
    frames: [Texture2D; 2],
    current_frame: usize,
    frame_timer: f32,
    playing: bool,
}

impl EmotionAnimation {
    pub fn new(animation_type: EmotionAnimationType) -> Self {
        Self {
            frames: Self::get_frames(animation_type),
            current_frame: 0,
            frame_timer: 0.0,
            playing: true,
        }
    }
    
    fn update_state(&mut self) {
        if self.frame_timer > 0.50 {
            self.current_frame += 1;
            self.frame_timer = 0.0;
        }

        self.frame_timer += get_frame_time();
        
        if self.current_frame >= 4 {
            self.playing = false;
        }
    }
    
    fn get_frames(animation_type: EmotionAnimationType) -> [Texture2D; 2] {
        match animation_type {
            EmotionAnimationType::Love => [
                include_texture!("../../resources/animations/emotions/love0.png"),
                include_texture!("../../resources/animations/emotions/love1.png"),
            ],
            EmotionAnimationType::Sad => [
                include_texture!("../../resources/animations/emotions/sad0.png"),
                include_texture!("../../resources/animations/emotions/sad1.png"),
            ],
            EmotionAnimationType::Tired => [
                include_texture!("../../resources/animations/emotions/tired0.png"),
                include_texture!("../../resources/animations/emotions/tired1.png"),
            ],
        }
    }
}

impl Animation for EmotionAnimation {
    fn render(&mut self) {
        let draw_location = self.frame_draw_location();

        self.draw_background();
        draw_texture(
            &self.frames[self.current_frame % 2],
            draw_location.x,
            draw_location.y,
            BLACK,
        );

        self.update_state();
    }

    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.frames[0].width(),
            height: self.frames[0].height(),
        }
    }

    fn playing(&self) -> bool {
        self.playing
    }
}

impl PopupAnimation for EmotionAnimation {}
