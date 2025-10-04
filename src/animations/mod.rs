use macroquad::prelude::*;
use crate::{BACKGROUND_COLOR, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::utils::Location;

pub mod creature_eat;
pub mod creature_health;

/// An animation that can be rendered on the screen.
pub trait Animation {
    /// Calling this method should render the appropriate frame of the animation.
    fn render(&mut self);

    /// Returns the animations dimensions as `[x, y]`
    fn dimensions(&self) -> [f32; 2];

    /// Returns `true` if the animation is still playing, `false` otherwise.
    fn playing(&self) -> bool;
}

pub trait PopupAnimation: Animation {
    /// Draw the background for this PopupAnimation, by default this is a rectangle with
    /// the color `crate::BACKGROUND_COLOR`.
    fn draw_background(&self) {
        let width = self.dimensions()[0] + 10.0;
        let height = self.dimensions()[1] + 10.0;

        draw_rectangle(
            SCREEN_WIDTH as f32 / 2.0 - width / 2.0,
            SCREEN_HEIGHT as f32 / 2.0 - height / 2.0 - SCREEN_HEIGHT as f32 / 4.0,
            width,
            height,
            BACKGROUND_COLOR,
        );
    }

    /// Returns the location where the animation's frame should be drawn. By default, this is the
    /// center of the screen, while taking account for the animation's size.
    fn frame_draw_location(&self) -> Location {
        Location {
            x: SCREEN_WIDTH as f32 / 2.0 - self.dimensions()[0] / 2.0,
            y: SCREEN_HEIGHT as f32 / 2.0 - self.dimensions()[1] / 2.0 - SCREEN_HEIGHT as f32 / 4.0,
        }
    }
}