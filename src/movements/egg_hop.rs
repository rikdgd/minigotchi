use macroquad::time::get_frame_time;
use crate::movements::CreatureMovement;
use crate::utils::Location;

const SPRITE_DIMENSION: f32 = 10.0;

pub struct EggHop {
    base_location: Location,
    is_grounded: bool,
    timer: f32,
}

impl EggHop {
    pub fn new(base_location: Location) -> Self {
        Self {
            base_location: base_location.translate(
                (-SPRITE_DIMENSION / 2.0).round(),
                (-SPRITE_DIMENSION / 2.0).round(),
            ),
            is_grounded: true,
            timer: 0.0,
        }
    }
}

impl CreatureMovement for EggHop {
    fn next_position(&mut self) -> Location {
        // Update the animation timer
        self.timer += get_frame_time();
        if self.timer > 1.0 {
            self.is_grounded = !self.is_grounded;
            self.timer = 0.0;
        }
        
        match self.is_grounded {
            true => self.base_location,
            false => self.base_location.translate(0.0, 5.0),
        }
    }

    fn current_position(&self) -> Location {
        match self.is_grounded {
            true => self.base_location,
            false => self.base_location.translate(0.0, 5.0),
        }
    }

    fn mirror_sprite(&self) -> bool {
        false
    }
}