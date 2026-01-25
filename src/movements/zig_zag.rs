use crate::movements::CreatureMovement;
use crate::utils::Location;
use macroquad::time::get_frame_time;

/// This is the sprite size for the sprite used in the `ZigZag` movement, which is 
/// 25x25 pixels.
const SPRITE_SIZE: f32 = 15.0;

/// The movement that should be displayed when the creature is in the **"Kid"** growth
/// stage.
pub struct ZigZag {
    base_location: Location,
    x_shift: f32,
    y_shift: f32,
    x_toggle: bool,
    y_toggle: bool,
    timer: f32,
}

impl ZigZag {
    /// Updates the base location, while keeping account for the sprite's size.
    pub fn base_location(mut self, location: Location) -> Self {
        self.base_location = location.translate(
            (-SPRITE_SIZE / 2.0).round(),
            (-SPRITE_SIZE / 2.0).round(),
        );
        self
    }
    
    /// Updates the state of the movement to its next position. It doesn't account for the amount 
    /// of time that has passed.
    fn update_state(&mut self) {
        match self.x_toggle {
            true => self.x_shift += 1.0,
            false => self.x_shift -= 1.0,
        }
        
        match self.y_toggle {
            true => self.y_shift += 1.0,
            false => self.y_shift -= 1.0,
        }
        
        if self.x_shift.abs() >= 10.0 {
            self.x_toggle = !self.x_toggle;
        }
        
        if self.y_shift.abs() >= 2.0 {
            self.y_toggle = !self.y_toggle;
        }
    }
}

impl Default for ZigZag {
    fn default() -> Self {
        let center_screen = Location {
            x: 100.0, 
            y: 100.0,
        };
        
        Self {
            base_location: center_screen.translate(
                SPRITE_SIZE / 2.0, 
                SPRITE_SIZE / 2.0
            ),
            x_shift: 0.0,
            y_shift: 0.0,
            x_toggle: true,
            y_toggle: true,
            timer: 0.0,
        }
    }
}

impl CreatureMovement for ZigZag {
    fn next_position(&mut self) -> Location {
        self.timer += get_frame_time();
        if self.timer > 0.25 {
            self.update_state();
            self.timer = 0.0;
        }
        
        Location {
            x: self.base_location.x + self.x_shift,
            y: self.base_location.y + self.y_shift,
        }
    }

    fn current_position(&self) -> Location {
        Location {
            x: self.base_location.x + self.x_shift,
            y: self.base_location.y + self.y_shift,
        }
    }

    fn mirror_sprite(&self) -> bool {
        self.x_toggle
    }
}