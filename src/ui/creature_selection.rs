use macroquad::prelude::*;
use crate::shapes::CreatureShape;
use crate::ui::button::Button;
use crate::{BACKGROUND_COLOR, SCREEN_WIDTH, SCREEN_HEIGHT};


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
    selected_shape: CreatureShape,
    next_btn: Button,
    confirm_btn: Button,
}

impl CreatureSelection {
    const CREATURE_ZOOM_FACTOR: f32 = 1.5;
    const INFO_TEXT: &str = "Select a creature:";
    const FONT_SIZE: f32 = 20.0;
    
    /// Renders and updates the state of the creature selection screen.
    ///
    /// ## Returns:
    /// When the user has confirmed their choice of shape, this method will return a copy of
    /// `self.selected_shape`.
    pub async fn render(&mut self) -> CreatureShape {
        let creature_shape: CreatureShape;
        let info_text_dimensions = measure_text(
            Self::INFO_TEXT, 
            None, 
            Self::FONT_SIZE as u16, 
            1.0
        );
        
        loop {
            clear_background(BACKGROUND_COLOR);

            self.next_btn.render();
            self.confirm_btn.render();

            Self::draw_info_text(info_text_dimensions);
            self.draw_creature_texture();

            // Update the menu's state and when the user picked a shape, break the render loop.
            if let Some(shape) = self.update() {
                creature_shape = shape;
                break;
            }

            next_frame().await
        }
        
        creature_shape
    }
    
    fn draw_info_text(dimensions: TextDimensions) {
        draw_text(
            Self::INFO_TEXT,
            SCREEN_WIDTH as f32 / 2.0 - dimensions.width / 2.0,
            (SCREEN_HEIGHT as f32 / 2.0 - dimensions.height / 2.0) - 50.0,
            Self::FONT_SIZE,
            BLACK,
        );
    }
    
    fn draw_creature_texture(&self) {
        let creature_texture = self.selected_shape.get_texture();
        draw_texture_ex(
            &creature_texture,
            SCREEN_WIDTH as f32 / 2.0 - (creature_texture.width() * Self::CREATURE_ZOOM_FACTOR) / 2.0,
            (SCREEN_HEIGHT as f32 / 2.0 - (creature_texture.height() * Self::CREATURE_ZOOM_FACTOR) / 2.0) - 10.0,
            BLACK,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    creature_texture.width() * Self::CREATURE_ZOOM_FACTOR,
                    creature_texture.height() * Self::CREATURE_ZOOM_FACTOR,
                )),
                ..Default::default()
            },
        );
    }

    fn update(&mut self) -> Option<CreatureShape> {
        if self.next_btn.is_clicked() {
            self.selected_shape = Self::next_creature(self.selected_shape);
        }

        if self.confirm_btn.is_clicked() {
            return Some(self.selected_shape);
        }

        None
    }

    fn next_creature(creature: CreatureShape) -> CreatureShape {
        match creature {
            CreatureShape::Turtle => CreatureShape::Snail,
            CreatureShape::Snail => CreatureShape::Fish,
            CreatureShape::Fish => CreatureShape::Mouse,
            CreatureShape::Mouse => CreatureShape::Frog,
            CreatureShape::Frog => CreatureShape::Squid,
            CreatureShape::Squid => CreatureShape::Sheep,
            CreatureShape::Sheep => CreatureShape::Germ,
            CreatureShape::Germ => CreatureShape::Jellyfish,
            CreatureShape::Jellyfish => CreatureShape::Turtle,
        }
    }

    fn next_button() -> Button {
        let mut next_btn = Button::default();
        next_btn.text = "next".to_string();
        next_btn.pos = (
            ((SCREEN_WIDTH as f32 / 2.0) - next_btn.size.x / 2.0) - 30.0,
            ((SCREEN_HEIGHT as f32 / 2.0) - next_btn.size.y / 2.0) + 50.0,
        ).into();

        next_btn
    }

    fn confirm_btn() -> Button {
        let mut confirm_btn = Button::default();
        confirm_btn.text = "confirm".to_string();
        confirm_btn.pos = (
            ((SCREEN_WIDTH as f32 / 2.0) - confirm_btn.size.x / 2.0) + 30.0,
            ((SCREEN_HEIGHT as f32 / 2.0) - confirm_btn.size.y / 2.0) + 50.0,
        ).into();

        confirm_btn
    }
}

impl Default for CreatureSelection {
    fn default() -> Self {
        Self {
            selected_shape: CreatureShape::new_random(),
            next_btn: Self::next_button(),
            confirm_btn: Self::confirm_btn(),
        }
    }
}
