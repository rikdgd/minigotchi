use macroquad::prelude::*;
use crate::creature::Creature;
use crate::movements::CreatureMovement;
use crate::utils::{Dimensions, Location};
use crate::ui::play_area::PLAY_AREA_RECT;

const MOVE_SPEED: f32 = 1.0;
const FRAME_TIME: f32 = 0.25;

/// # CursorStalk
/// A `CreatureMovement` that makes the creature move towards the mouse cursor at a constant speed.
/// The creature stays within the playing field and will not leave it.
#[derive(Debug, Clone, Copy)]
pub struct CursorStalk {
    pub current_location: Location,
    last_x_movement: f32,
    shape_dimensions: Dimensions,
    timer: f32,
}

impl CursorStalk {
    pub fn new(start_location: Location, creature: &Creature) -> CursorStalk {
        CursorStalk {
            current_location: start_location,
            last_x_movement: 0.0,
            shape_dimensions: Dimensions {
                width: creature.shape().width(),
                height: creature.shape().height(),
            },
            timer: 0.0,
        }
    }

    fn update_state(&mut self) {
        let mouse_pos: Vec2 = mouse_position().into();
        let (x_move, y_move) = self.calc_xy_movement(&mouse_pos);

        self.current_location.x += x_move;
        self.current_location.y += y_move;
        self.validify_location();
    }

    fn calc_mouse_distance(&self, mouse_pos: &Vec2) -> f32 {
        let x_dist = mouse_pos.x - self.current_location.x;
        let y_dist = mouse_pos.y - self.current_location.y;

        (x_dist.powi(2) + y_dist.powi(2)).sqrt()
    }

    fn calc_xy_movement(&self, mouse_pos: &Vec2) -> (f32, f32) {
        let x_dist = mouse_pos.x - self.current_location.x;
        let y_dist = mouse_pos.y - self.current_location.y;
        let mouse_dist = self.calc_mouse_distance(mouse_pos);

        (
            x_dist / mouse_dist * MOVE_SPEED,
            y_dist / mouse_dist * MOVE_SPEED,
        )
    }

    /// Checks if `self.current_location` is still within the bounds of the playing area, and if not,
    /// sets it's location to the nearest one inside of bounds.
    fn validify_location(&mut self) {
        let left_limit = PLAY_AREA_RECT.left();
        if self.current_location.x <= left_limit {
            self.current_location.x = left_limit;
        }

        let right_limit = PLAY_AREA_RECT.right() - self.shape_dimensions.width;
        if self.current_location.x >= right_limit {
            self.current_location.x = right_limit;
        }

        let top_limit = PLAY_AREA_RECT.top();
        if self.current_location.y <= top_limit {
            self.current_location.y = top_limit;
        }

        let bottom_limit = PLAY_AREA_RECT.bottom() - self.shape_dimensions.height;
        if self.current_location.y >= bottom_limit {
            self.current_location.y = bottom_limit;
        }
    }
}

impl CreatureMovement for CursorStalk {
    fn next_position(&mut self) -> Location {
        self.timer += get_frame_time();
        if self.timer >= FRAME_TIME {
            self.update_state();
            self.timer = 0.0;
        }

        self.current_location
    }

    fn mirror_sprite(&self) -> bool {
        self.last_x_movement >= 0.0
    }
}