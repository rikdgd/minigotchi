use macroquad::prelude::*;
use crate::movements::CreatureMovement;
use crate::ui::play_area::play_area_center;
use crate::utils::Location;

#[derive(Debug, Clone, Copy)]
pub struct SicknessShakeMovement {
    current_location: Location,
    is_shaking: bool,
    shake_toggle: bool,
    timer: f32,
}

impl SicknessShakeMovement {
    fn update_state(&mut self) {
        self.timer += get_frame_time();
        
        if self.is_shaking {
            if self.timer >= 0.25 {
                self.is_shaking = false;
                self.timer = 0.0;
            }

            if (self.timer % 5.0).round() == 0.0 {
                self.shake_toggle = !self.shake_toggle;
            }

            if self.shake_toggle {
                self.current_location.x += 5.0;
            } else {
                self.current_location.x -= 5.0;
            }
        } else {
            if self.timer >= 0.5 {
                self.is_shaking = true;
                self.timer = 0.0;
            }

            self.current_location = play_area_center();
        }
    }
}

impl Default for SicknessShakeMovement {
    fn default() -> Self {
        Self {
            current_location: play_area_center(),
            is_shaking: false,
            shake_toggle: false,
            timer: 0.0
        }
    }
}

impl CreatureMovement for SicknessShakeMovement {
    fn current_location(&self) -> Location {
        self.current_location
    }

    fn next_location(&mut self) -> Location {
        self.update_state();
        self.current_location
    }

    fn mirror_sprite(&self) -> bool {
        false
    }
}