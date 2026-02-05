use macroquad::prelude::*;
use crate::creature::Creature;
use crate::movements::CreatureMovement;
use crate::ui::play_area::play_area_center;
use crate::utils::{Dimensions, Location};

#[derive(Debug, Clone, Copy)]
pub struct SicknessShakeMovement {
    current_location: Location,
    creature_shape_dimensions: Dimensions,
    is_shaking: bool,
    shake_toggle: bool,
    timer: f32,
}

impl SicknessShakeMovement {
    pub fn new(creature: &Creature) -> Self {
        Self {
            current_location: play_area_center(),
            creature_shape_dimensions: Dimensions {
                width: creature.shape().width(),
                height: creature.shape().height(),
            },
            is_shaking: false,
            shake_toggle: false,
            timer: 0.0
        }
    }

    fn update_state(&mut self) {
        self.timer += get_frame_time();

        if self.is_shaking {
            // Stop the creature from rapidly shaking
            if self.timer >= 0.25 {
                self.is_shaking = false;
                self.timer = 0.0;
            }

            // Make the creature shake by adjusting its location
            if (self.timer % 0.2) == 0.0 {
                self.shake_toggle = !self.shake_toggle;

                if self.shake_toggle {
                    self.current_location.x += 5.0;
                } else {
                    self.current_location.x -= 5.0;
                }
            }
        // When the creature is not shaking:
        } else {
            // Start the shaking movement after 0.5 seconds of standing still
            if self.timer >= 0.5 {
                self.is_shaking = true;
                self.timer = 0.0;
            }

            self.current_location = play_area_center();
        }
    }
}

impl CreatureMovement for SicknessShakeMovement {
    fn current_location(&self) -> Location {
        self.current_location
    }

    fn next_location(&mut self) -> Location {
        self.update_state();
        Location {
            x: (self.current_location.x - self.creature_shape_dimensions.width / 2.0).round(),
            y: (self.current_location.y - self.creature_shape_dimensions.height / 2.0).round(),
        }
    }

    fn mirror_sprite(&self) -> bool {
        false
    }
}