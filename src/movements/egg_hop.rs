use macroquad::time::get_frame_time;
use crate::movements::CreatureMovement;
use crate::utils::Location;

pub struct EggHop {
    base_location: Location,
    is_grounded: bool,
    timer: f32,
}

impl EggHop {
    pub fn new(base_location: Location) -> Self {
        Self {
            base_location,
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
}