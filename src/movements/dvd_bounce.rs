use macroquad::time::get_frame_time;
use crate::creature::Creature;
use crate::movements::CreatureMovement;
use crate::utils::{Dimensions, Location};
use crate::utils::direction::{XDirection, YDirection};
use crate::ui::play_area::PLAY_AREA_RECT;

const STEP_SIZE: f32 = 1.0;

/// A `CreatureMovement` that makes the creature bounce around in a "box" like the dvd logo.
pub struct DvdBounce {
    timer: f32,
    creature_location: Location,
    x_direction: XDirection,
    y_direction: YDirection,
    shape_dimensions: Dimensions,
}

impl DvdBounce {
    pub fn new(creature: &Creature) -> Self {
        let shape = creature.texture();
        Self {
            timer: 0.0,
            creature_location: Location { x: 50.0, y: 50.0 },
            x_direction: XDirection::new_random(),
            y_direction: YDirection::new_random(),
            shape_dimensions: Dimensions {
                width: shape.width(),
                height: shape.height(),
            },
        }
    }

    /// **Builder pattern** setter for the starting location of this movement.
    pub fn start_location(mut self, location: Location) -> Self {
        self.creature_location = location;
        self
    }

    /// Updates the movement toggles `self.x_toggle` and `self.y_toggle` when the creature is about to move out of bounds.
    fn update_toggles(&mut self) {
        if self.creature_location.x >= PLAY_AREA_RECT.right() - self.shape_dimensions.width {
            self.x_direction = XDirection::Left;
        } else if self.creature_location.x <= PLAY_AREA_RECT.x {
            self.x_direction = XDirection::Right;
        }
        
        if self.creature_location.y >= PLAY_AREA_RECT.bottom() - self.shape_dimensions.height {
            self.y_direction = YDirection::Up;
        } else if self.creature_location.y <= PLAY_AREA_RECT.y {
            self.y_direction = YDirection::Down;
        }
    }

    /// Updates the state of the movement, this does **not** keep track of the amount of time
    /// passed since the previous update.
    fn update_state(&mut self) {
        self.update_toggles();
        
        match self.x_direction {
            XDirection::Right => self.creature_location.x += STEP_SIZE,
            XDirection::Left => self.creature_location.x -= STEP_SIZE,
        }

        match self.y_direction {
            YDirection::Up => self.creature_location.y -= STEP_SIZE,
            YDirection::Down => self.creature_location.y += STEP_SIZE,
        }
    }
}

impl CreatureMovement for DvdBounce {
    fn current_location(&self) -> Location {
        self.creature_location
    }

    fn next_location(&mut self) -> Location {
        self.timer += get_frame_time();
        if self.timer > 0.25 {
            self.update_state();
            self.timer = 0.0;
        }
        
        self.creature_location
    }

    fn mirror_sprite(&self) -> bool {
        self.x_direction == XDirection::Right
    }
}