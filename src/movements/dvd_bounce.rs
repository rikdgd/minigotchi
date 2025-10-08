use macroquad::time::get_frame_time;
use crate::movements::CreatureMovement;
use crate::utils::Location;
use crate::ui::play_area::PLAY_AREA_RECT;

const SHAPE_DIMENSION: f32 = 25.0;
const STEP_SIZE: f32 = 1.0;

/// A `CreatureMovement` that makes the creature bounce around in a "box" like the dvd logo.
pub struct DvdBounce {
    timer: f32,
    creature_location: Location,
    x_toggle: bool,
    y_toggle: bool,
}

impl DvdBounce {
    pub fn new() -> Self {
        // TODO: Randomize the starting location and direction
        Self {
            timer: 0.0,
            creature_location: Location { x: 50.0, y: 50.0 },
            x_toggle: true,
            y_toggle: true,
        }
    }

    /// Updates the movement toggles when the creature is about to move out of bounds.
    fn update_toggles(&mut self) {
        if self.creature_location.y >= PLAY_AREA_RECT.bottom() - SHAPE_DIMENSION ||
            self.creature_location.y <= PLAY_AREA_RECT.y
        {
            self.y_toggle = !self.y_toggle;
        }

        if self.creature_location.x >= PLAY_AREA_RECT.right() - SHAPE_DIMENSION ||
            self.creature_location.x <= PLAY_AREA_RECT.x
        {
            self.x_toggle = !self.x_toggle;
        }
    }

    /// Updates the state of the movement, this does **not** keep track of the amount of time
    /// passed since the previous update.
    fn update_state(&mut self) {
        self.update_toggles();

        match self.x_toggle {
            true => self.creature_location.x += STEP_SIZE,
            false => self.creature_location.x -= STEP_SIZE,
        }

        match self.y_toggle {
            true => self.creature_location.y += STEP_SIZE,
            false => self.creature_location.y -= STEP_SIZE,
        }
    }

    pub fn moving_right(&self) -> bool {
        self.x_toggle
    }

    pub fn moving_down(&self) -> bool {
        self.y_toggle
    }
}

impl CreatureMovement for DvdBounce {
    fn next_position(&mut self) -> Location {
        self.timer += get_frame_time();
        if self.timer > 0.25 {
            self.update_state();
            self.timer = 0.0;
        }

        self.creature_location
    }

    fn mirror_sprite(&self) -> bool {
        self.x_toggle
    }
}