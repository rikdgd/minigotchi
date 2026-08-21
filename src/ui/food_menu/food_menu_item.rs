use macroquad::prelude::*;
use crate::food::Food;
use crate::include_texture;
use crate::ui::food_menu::ITEM_CONTAINER_AREA;
use crate::utils::Dimensions;


/// The **FoodMenuItem** struct is a UI component used by the `FoodMenu` component. It can be clicked
/// by the user to select the food they want to feed their creature.
/// 
/// `FoodMenuItem` instances **should not** be constructed by hand. They are instead created by the
/// `minigotchi::ui::food_menu::food_menu_item::gen_all_food_items` function.
/// 
/// ## Fields:
/// * `food` - The **Food** that this specific item represents.
/// * `area` - The area of the menu item, used for rendering and click/hover detection.
#[derive(Debug, Clone, Copy)]
pub struct FoodMenuItem {
    pub food: Food,
    pub area: Rect,
}

impl FoodMenuItem {
    pub const DIMENSIONS: Dimensions = Dimensions { width: 130.0, height: 30.0 };

    pub fn new(food: Food) -> Self {
        Self {
            food,
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
            self.food.name(),
            None,
            16,
            1.0,
        );
        
        draw_text(
            self.food.name(),
            self.area.x + (self.area.w - txt_size.width) / 2.0,
            (self.area.y + txt_size.height + (self.area.h - txt_size.height) / 2.0).round(),
            16.0,
            BLACK,
        );
    }
    
    fn draw_sprite(&self) {
        let sprite_size = Vec2::new(20.0, 20.0);
        let sprite = match self.food {
            Food::Soup => include_texture!("../../../resources/animations/eating/soup0.png"),
            Food::Cookie => include_texture!("../../../resources/animations/eating/cookie0.png"),
            Food::Burger => include_texture!("../../../resources/animations/eating/burger0.png"),
        };
        
        draw_texture_ex(
            &sprite,
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
