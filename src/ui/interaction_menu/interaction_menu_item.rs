use macroquad::prelude::*;
use crate::ui::interaction_menu::{CreatureInteraction, ITEM_CONTAINER_AREA};
use crate::utils::Dimensions;


/// The **InteractionMenuItem** struct is a UI component used by the `InteractionMenu` component.
/// It can be clicked by the user to select the interaction they want to perform on their creature.
/// 
/// ## Fields:
/// * `interaction` - The **Interaction** that this specific item represents.
/// * `area` - The area of the menu item, used for rendering and click/hover detection.
#[derive(Debug, Clone, Copy)]
pub struct InteractionMenuItem<T>
where
T: CreatureInteraction
{
    pub interaction: T,
    pub area: Rect,
}

impl<T> InteractionMenuItem<T>
where
    T: CreatureInteraction
{
    pub const DIMENSIONS: Dimensions = Dimensions { width: 130.0, height: 30.0 };

    pub fn new(interaction: T) -> Self {
        Self {
            interaction,
            area: Rect::new(
                (ITEM_CONTAINER_AREA.w - Self::DIMENSIONS.width) / 2.0 + ITEM_CONTAINER_AREA.x,
                0.0,
                Self::DIMENSIONS.width,
                Self::DIMENSIONS.height,
            ),
        }
    }

    pub fn draw(&self) {
        let container_color = if self.is_hovered() {
            Color { r: 0.55, g: 0.55, b: 0.55, a: 1.0 }
        } else {
            Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 }
        };
        draw_rectangle(
            self.area.x,
            self.area.y,
            self.area.w,
            self.area.h,
            container_color,
        );
        
        self.draw_sprite();
        self.draw_name();
    }

    pub fn is_clicked(&self) -> bool {
        self.is_hovered() && is_mouse_button_pressed(MouseButton::Left)
    }
    
    fn is_hovered(&self) -> bool {
        self.area.contains(mouse_position().into())
    }
    
    fn draw_name(&self) {
        let txt_size = measure_text(
            self.interaction.name(),
            None,
            16,
            1.0,
        );
        
        draw_text(
            self.interaction.name(),
            self.area.x + (self.area.w - txt_size.width) / 2.0,
            (self.area.y + txt_size.height + (self.area.h - txt_size.height) / 2.0).round(),
            16.0,
            BLACK,
        );
    }
    
    fn draw_sprite(&self) {
        let sprite_size = Vec2::new(20.0, 20.0);
        draw_texture_ex(
            &self.interaction.sprite(),
            self.area.x + 5.0,
            (self.area.y + (self.area.h - sprite_size.y) / 2.0).round(),
            BLACK,
            DrawTextureParams {
                dest_size: Some(sprite_size),
                ..Default::default()
            },
        );
    }
}
