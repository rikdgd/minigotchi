use macroquad::time::get_frame_time;
use crate::movements::CreatureMovement;
use crate::utils::Location;

const SHAPE_DIMENSION: f32 = 25.0;
const STEP_SIZE: f32 = 1.0;

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

    fn update_toggles(&mut self) {
        if self.creature_location.y >= 100.0 - SHAPE_DIMENSION ||
            self.creature_location.y <= 0.0
        {
            self.y_toggle = !self.y_toggle;
        }

        if self.creature_location.x >= crate::SCREEN_WIDTH as f32 - SHAPE_DIMENSION ||
            self.creature_location.x <= 0.0
        {
            self.x_toggle = !self.x_toggle;
        }
    }

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
}