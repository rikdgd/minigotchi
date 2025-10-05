use macroquad::prelude::*;
use super::{Animation, PopupAnimation};
use crate::include_texture;

/// An enum used with the `CreatureActionAnimation` struct to tell it which animation to render. 
/// There are multiple different animations that can be rendered, mapped to the following values:
/// * `ActionAnimationType::Eating` - This is the animation that plays when feeding the creature.
/// * `ActionAnimationType::Health` - The animation that plays when the creature's health stat is increased
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ActionAnimationType {
    Eating,
    Health,
}

/// The `PopupAnimation` that is used to display animations for *actions* the user can take to take
/// care for the creature. These include feeding it and giving it medicine.
#[derive(Debug, Clone)]
pub struct CreatureActionAnimation {
    current_frame: usize,
    frame_timer: f32,
    playing: bool,
    frames: [Texture2D; 4],
    dimensions: [f32; 2],
}

impl Animation for CreatureActionAnimation {
    fn render(&mut self) {
        let draw_location = self.frame_draw_location();

        self.draw_background();
        draw_texture(
            &self.frames[self.current_frame],
            draw_location.x,
            draw_location.y,
            BLACK,
        );

        self.update_state();
    }

    fn dimensions(&self) -> [f32; 2] {
        self.dimensions
    }

    fn playing(&self) -> bool {
        self.playing
    }
}

impl PopupAnimation for CreatureActionAnimation {}

impl CreatureActionAnimation {
    pub fn new(action_type: ActionAnimationType) -> Self {
        let frames = Self::get_frames(action_type);
        let dimensions = [
            frames[0].width(),
            frames[0].height(),
        ];

        Self {
            current_frame: 0,
            frame_timer: 0.0,
            playing: true,
            frames,
            dimensions,
        }
    }

    fn update_state(&mut self) {
        if self.frame_timer > 0.75 {
            self.current_frame += 1;
            self.frame_timer = 0.0;
        }

        self.frame_timer += get_frame_time();

        if self.current_frame >= self.frames.len() {
            self.playing = false;
        }
    }
    
    fn get_frames(action_type: ActionAnimationType) -> [Texture2D; 4] {
        match action_type {
            ActionAnimationType::Eating => 
            [
                include_texture!("../../resources/animations/eating/burger0.png"),
                include_texture!("../../resources/animations/eating/burger1.png"),
                include_texture!("../../resources/animations/eating/burger2.png"),
                include_texture!("../../resources/animations/eating/burger3.png"),
            ],
            
            ActionAnimationType::Health => 
            [
                include_texture!("../../resources/animations/health/syringe0.png"),
                include_texture!("../../resources/animations/health/syringe1.png"),
                include_texture!("../../resources/animations/health/syringe2.png"),
                include_texture!("../../resources/animations/health/syringe3.png"),
            ]
        }
    }
}
