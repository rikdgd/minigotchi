use macroquad::prelude::*;
use crate::{BACKGROUND_COLOR, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::utils::{Location, Dimensions};

pub mod creature_actions;
mod emotions;

/// An animation that can be rendered on the screen.
pub trait Animation {
    /// Calling this method renders the appropriate frame of the animation, and updates the
    /// animation's state.
    fn render(&mut self);

    /// Returns the animation's dimensions in pixels.
    fn dimensions(&self) -> Dimensions;

    /// Returns `true` if the animation is still playing, `false` otherwise.
    fn playing(&self) -> bool;
}

pub trait PopupAnimation: Animation {
    /// Draw the background for this PopupAnimation, by default this is a rectangle with
    /// the color `crate::BACKGROUND_COLOR`.
    fn draw_background(&self) {
        let width = self.dimensions().width + 10.0;
        let height = self.dimensions().height + 10.0;

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
            x: SCREEN_WIDTH as f32 / 2.0 - self.dimensions().width / 2.0,
            y: SCREEN_HEIGHT as f32 / 2.0 - self.dimensions().height / 2.0 - SCREEN_HEIGHT as f32 / 4.0,
        }
    }
}