use macroquad::color::Color;
use macroquad::math::Rect;
use macroquad::prelude::{draw_rectangle, measure_text};
use crate::food::Food;
use crate::ui::food_menu::ITEM_CONTAINER_AREA;
use crate::utils::Dimensions;

#[derive(Debug, Clone, Copy)]
pub struct FoodMenuItem {
    pub food: Food,
    area: Rect,
}

impl FoodMenuItem {
    const DIMENSIONS: Dimensions = Dimensions { width: 130.0, height: 50.0 };

    pub fn new(food: Food, draw_height: f32) -> Self {
        Self {
            food,
            area: Rect::new(
                (ITEM_CONTAINER_AREA.w - Self::DIMENSIONS.width) / 2.0 + ITEM_CONTAINER_AREA.x,
                draw_height,
                Self::DIMENSIONS.width,
                Self::DIMENSIONS.height,
            ),
        }
    }

    pub fn draw(&self) {
        let name_dimensions = measure_text(
            self.food.name(),
            None,
            16,
            1.0,
        );

        draw_rectangle(
            self.area.x,
            self.area.y,
            self.area.w,
            self.area.h,
            Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 },
        );
    }

    pub fn is_clicked(&self) -> bool {
        // todo!()
        false
    }
}
