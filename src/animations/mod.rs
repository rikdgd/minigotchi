use macroquad::prelude::*;
use crate::{BACKGROUND_COLOR, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::utils::Location;

pub mod creature_eat;

/// An animation that can be rendered on the screen.
pub trait Animation {
    /// Calling this method should render the appropriate frame of the animation.
    async fn render(&mut self);

    /// Returns the animations dimensions as `[x, y]`
    fn dimensions(&self) -> [f32; 2];
}

pub trait PopupAnimation: Animation {
    fn draw_background(&self) {
        let width = self.dimensions()[0] + 10.0;
        let height = self.dimensions()[1] + 10.0;

        draw_rectangle(
            SCREEN_WIDTH as f32 / 2.0 - width / 2.0,
            SCREEN_HEIGHT as f32 / 2.0 - height / 2.0,
            width,
            height,
            BACKGROUND_COLOR,
        );
    }

    fn frame_draw_location(&self) -> Location {
        Location {
            x: SCREEN_WIDTH as f32 / 2.0 - self.dimensions()[0] / 2.0,
            y: SCREEN_HEIGHT as f32 / 2.0 - self.dimensions()[1] / 2.0,
        }
    }
}