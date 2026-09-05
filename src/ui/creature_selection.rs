use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::shapes::CreatureShape;
use crate::ui::button::Button;
use crate::{BACKGROUND_COLOR, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::ui::new_game_menu::NewGameMenu;


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
    selection_index: isize,
    previous_btn: Button,
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
            self.previous_btn.render();

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
        let creature_texture = self.selected_shape().get_texture();
        draw_texture_ex(
            &creature_texture,
            (SCREEN_WIDTH as f32 - (creature_texture.width() * Self::CREATURE_ZOOM_FACTOR)) / 2.0,
            (SCREEN_HEIGHT as f32 - creature_texture.height() * Self::CREATURE_ZOOM_FACTOR) / 2.0 - 10.0,
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
            self.selection_index += 1;
        }
        
        if self.previous_btn.is_clicked() {
            self.selection_index -= 1;
        }

        if self.confirm_btn.is_clicked() {
            return Some(self.selected_shape());
        }

        None
    }
    
    fn selected_shape(&self) -> CreatureShape {
        let creature_index = self.selection_index % CreatureShape::ALL_VARIANTS.len() as isize;
        let creature_index = if creature_index < 0 {
            CreatureShape::ALL_VARIANTS.len() as isize + creature_index
        } else {
            creature_index
        };
        CreatureShape::ALL_VARIANTS[creature_index as usize]
    }

    fn next_button() -> Button {
        let mut next_btn = Button::default();
        next_btn.text = ">".to_string();
        next_btn.fontsize = 16.0;

        next_btn.size = (15.0, 20.0).into();
        next_btn.pos = (
            (SCREEN_WIDTH as f32 - next_btn.size.x) / 2.0 + 50.0,
            SCREEN_HEIGHT as f32 / 2.0 - next_btn.size.y
        ).into();
        
        next_btn
    }

    fn previous_button() -> Button {
        let mut btn = Button::default();
        btn.text = "<".to_string();
        btn.fontsize = 16.0;
        
        btn.size = (15.0, 20.0).into();
        btn.pos = (
            (SCREEN_WIDTH as f32 - btn.size.x) / 2.0 - 50.0,
            SCREEN_HEIGHT as f32 / 2.0 - btn.size.y
        ).into();
        
        btn
    }

    fn confirm_btn() -> Button {
        NewGameMenu::confirm_btn()
    }
}

impl Default for CreatureSelection {
    fn default() -> Self {
        Self {
            selection_index: gen_range(0, CreatureShape::ALL_VARIANTS.len() as isize),
            next_btn: Self::next_button(),
            previous_btn: Self::previous_button(),
            confirm_btn: Self::confirm_btn(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::shapes::CreatureShape;
    use crate::ui::creature_selection::CreatureSelection;
    
    #[test]
    fn shape_selection_is_cycle() {
        let mut cs = CreatureSelection::default();
        let original_shape = cs.selected_shape();
        
        cs.selection_index += CreatureShape::ALL_VARIANTS.len() as isize;
        let new_shape = cs.selected_shape();
        
        assert_eq!(original_shape, new_shape);
        
        cs.selection_index -= CreatureShape::ALL_VARIANTS.len() as isize * 2;
        let new_shape = cs.selected_shape();
        assert_eq!(original_shape, new_shape);
    }
    
    #[test]
    fn selection_cycle_matches_shapes() {
        let mut cs = CreatureSelection {
            selection_index: 0,
            ..Default::default()
        };
        
        for expected_shape in CreatureShape::ALL_VARIANTS {
            assert_eq!(expected_shape, cs.selected_shape());
            cs.selection_index += 1;
        }
    }
}
