use macroquad::prelude::*;
use crate::animations::{Animation, PopupAnimation};
use crate::utils::Dimensions;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmotionAnimationType {
    Happy,
    Sad,
}

#[derive(Debug, Clone)]
pub struct EmotionAnimation {
    frames: [Texture2D; 2],
    current_frame: usize,
    frame_timer: f32,
    playing: bool,
}

impl Animation for EmotionAnimation {
    fn render(&mut self) {
        let draw_location = self.frame_draw_location();
        
        self.draw_background();
        // TODO: Draw the current frame texture
        
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
        
        if self.current_frame >= 10 {
            self.playing = false;
        }
    }
    
    fn get_frames(animation_type: EmotionAnimationType) -> [Texture2D; 2] {
        todo!()
    }
}
