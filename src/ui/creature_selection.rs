use macroquad::prelude::*;
use crate::shapes::CreatureShapes;
use crate::ui::button::Button;
use crate::{BACKGROUND_COLOR, SCREEN_WIDTH, SCREEN_HEIGHT};

const CREATURE_ZOOM_FACTOR: f32 = 1.5;


/// The **CreatureSelection** struct manages the state of the creature selection screen
/// when creating a new save. Whenever it is drawn to the screen it will automatically update its
/// state.
///
/// ## Fields:
/// * `selected_shape` - The shape that is currently selected in the in-game menu.
/// * `next_btn` - The button component that the user can use to select the next shape.
/// * `confirm_btn` - The button component that the user can use to select the next shape.
#[derive(Debug, Clone)]
pub struct CreatureSelection {
    selected_shape: CreatureShapes,
    next_btn: Button,
    confirm_btn: Button,
}

impl CreatureSelection {
    /// Renders and updates the state of the creature selection screen.
    pub async fn render(&mut self) -> CreatureShapes {
        let creature_shape: CreatureShapes;
        loop {
            clear_background(BACKGROUND_COLOR);

            self.next_btn.render();
            self.confirm_btn.render();

            let creature_texture = self.selected_shape.get_texture();
            draw_texture_ex(
                &creature_texture,
                SCREEN_WIDTH as f32 / 2.0 - (creature_texture.width() * CREATURE_ZOOM_FACTOR) / 2.0,
                (SCREEN_HEIGHT as f32 / 2.0 - (creature_texture.height() * CREATURE_ZOOM_FACTOR) / 2.0) - 35.0,
                BLACK,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(
                        creature_texture.width() * CREATURE_ZOOM_FACTOR,
                        creature_texture.height() * CREATURE_ZOOM_FACTOR,
                    )),
                    ..Default::default()
                },
            );

            // Update the menu's state and when the user picked a shape, break the render loop.
            if let Some(shape) = self.update() {
                creature_shape = shape;
                break;
            }

            next_frame().await
        }

        creature_shape
    }

    fn update(&mut self) -> Option<CreatureShapes> {
        if self.next_btn.is_clicked() {
            self.selected_shape = Self::next_creature(self.selected_shape);
        }

        if self.confirm_btn.is_clicked() {
            return Some(self.selected_shape);
        }

        None
    }

    fn next_creature(creature: CreatureShapes) -> CreatureShapes {
        match creature {
            CreatureShapes::Turtle => CreatureShapes::Snail,
            CreatureShapes::Snail => CreatureShapes::Fish,
            CreatureShapes::Fish => CreatureShapes::Mouse,
            CreatureShapes::Mouse => CreatureShapes::Frog,
            CreatureShapes::Frog => CreatureShapes::Squid,
            CreatureShapes::Squid => CreatureShapes::Sheep,
            CreatureShapes::Sheep => CreatureShapes::Turtle,
        }
    }

    fn next_button() -> Button {
        let mut next_btn = Button::default();
        next_btn.text = "next".to_string();
        next_btn.pos = (
            (SCREEN_WIDTH as f32 / 2.0) - next_btn.size.x / 2.0,
            ((SCREEN_HEIGHT as f32 / 2.0) - next_btn.size.y / 2.0) + 20.0,
        ).into();

        next_btn
    }

    fn confirm_btn() -> Button {
        let mut confirm_btn = Button::default();
        confirm_btn.text = "confirm".to_string();
        confirm_btn.pos = (
            (SCREEN_WIDTH as f32 / 2.0) - confirm_btn.size.x / 2.0,
            ((SCREEN_HEIGHT as f32 / 2.0) - confirm_btn.size.y / 2.0) + 45.0,
        ).into();

        confirm_btn
    }
}

impl Default for CreatureSelection {
    fn default() -> Self {
        Self {
            selected_shape: CreatureShapes::Turtle,
            next_btn: Self::next_button(),
            confirm_btn: Self::confirm_btn(),
        }
    }
}
