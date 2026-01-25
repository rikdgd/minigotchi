use macroquad::prelude::*;
use crate::creature::Creature;
use crate::movements::CreatureMovement;
use crate::utils::{Dimensions, Location};
use crate::ui::play_area::PLAY_AREA_RECT;

const MOVE_SPEED: f32 = 1.5;
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

        self.last_x_movement = x_move;
        self.current_location.x = (self.current_location.x + x_move).round();
        self.current_location.y = (self.current_location.y + y_move).round();
        self.validify_location();
    }
    
    /// Gets the `Location` representing the center of the sprite, accounting for the creature's
    /// location on the screen.
    fn sprite_center(&self) -> Location {
        Location {
            x: self.current_location.x + self.shape_dimensions.width / 2.0,
            y: self.current_location.y + self.shape_dimensions.height / 2.0,
        }
    }

    fn calc_mouse_distance(&self, mouse_pos: &Vec2) -> f32 {
        let creature_location = self.sprite_center();
        let x_dist = mouse_pos.x - creature_location.x;
        let y_dist = mouse_pos.y - creature_location.y;

        (x_dist.powi(2) + y_dist.powi(2)).sqrt()
    }

    fn calc_xy_movement(&self, mouse_pos: &Vec2) -> (f32, f32) {
        let creature_location = self.sprite_center();
        let x_dist = mouse_pos.x - creature_location.x;
        let y_dist = mouse_pos.y - creature_location.y;
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
    fn current_position(&self) -> Location {
        self.current_location
    }

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



#[cfg(test)]
mod test {
    use crate::movements::CursorStalk;
    use crate::utils::{Dimensions, Location};
    use crate::ui::play_area::PLAY_AREA_RECT;

    struct LocationExpectedPair {
        test_location: Location,
        expected_location: Location,
    }

    #[test]
    fn validify_location() {
        const TEST_SPRITE_DIMENSION: f32 = 10.0;
        let valid_location = Location {
            x: PLAY_AREA_RECT.left() + PLAY_AREA_RECT.w / 2.0,
            y: PLAY_AREA_RECT.top() + PLAY_AREA_RECT.h / 2.0,
        };

        let invalid_locations = vec![
            // Location is too low
            LocationExpectedPair {
                test_location: Location {
                    x: PLAY_AREA_RECT.left() + PLAY_AREA_RECT.w / 2.0,
                    y: PLAY_AREA_RECT.bottom(),
                },
                expected_location: Location {
                    x: PLAY_AREA_RECT.left() + PLAY_AREA_RECT.w / 2.0,
                    y: PLAY_AREA_RECT.bottom() - TEST_SPRITE_DIMENSION,
                },
            },
            // Location is too far left
            LocationExpectedPair {
                test_location: Location {
                    x: PLAY_AREA_RECT.left() - 1.0,
                    y: PLAY_AREA_RECT.top() + PLAY_AREA_RECT.h / 2.0
                },
                expected_location: Location {
                    x: PLAY_AREA_RECT.left(),
                    y: PLAY_AREA_RECT.top() + PLAY_AREA_RECT.h / 2.0,
                },
            },
            // Location is too low and too far to the right
            LocationExpectedPair {
                test_location: Location {
                    x: PLAY_AREA_RECT.right() + 1.0,
                    y: PLAY_AREA_RECT.bottom() + 5.0,
                },
                expected_location: Location {
                    x: PLAY_AREA_RECT.right() - TEST_SPRITE_DIMENSION,
                    y: PLAY_AREA_RECT.bottom() - TEST_SPRITE_DIMENSION,
                },
            },
        ];


        // Test the valid location
        let mut valid_movement = CursorStalk {
            current_location: valid_location,
            last_x_movement: 0.0,
            timer: 0.0,
            shape_dimensions: Dimensions { width: TEST_SPRITE_DIMENSION, height: TEST_SPRITE_DIMENSION },
        };
        valid_movement.validify_location();
        assert_eq!(valid_movement.current_location, valid_location);

        // Test invalid locations
        for test_pair in invalid_locations {
            let mut stalk_movement = CursorStalk {
                current_location: test_pair.test_location,
                last_x_movement: 0.0,
                timer: 0.0,
                shape_dimensions: Dimensions { width: TEST_SPRITE_DIMENSION, height: TEST_SPRITE_DIMENSION },
            };
            stalk_movement.validify_location();
            assert_eq!(stalk_movement.current_location, test_pair.expected_location);
        }
    }
}