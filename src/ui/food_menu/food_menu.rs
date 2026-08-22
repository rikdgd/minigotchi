use macroquad::prelude::*;
use crate::food::Food;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ui::button::Button;
use crate::ui::food_menu::{FoodMenuItem, ITEM_CONTAINER_AREA};


/// The **FoodMenu** struct is used to render a UI used to feed the creature.
/// 
/// ## Fields:
/// * `selected_food` - The food the user has selected to feed their creature.
/// * `running` - The running field is used to keep the render loop running. Setting it to false
///   stops the render loop. `true` by default.
/// * `menu_items` - These are the *button* UI elements the user can click to select a type of food.
/// * `return_btn` - A button UI component used to exit the **FoodMenu**.
/// 
/// ## Example:
/// ```rust
/// async fn example() {
///     // Render the food menu and receive the selected food:
///     let mut menu = FoodMenu::new();
///     let selected_food = menu.render().await;
/// 
///     if let Some(food) = selected_food {
///         // Feed the creature here
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FoodMenu {
    selected_food: Option<Food>,
    running: bool,
    menu_items: [FoodMenuItem; 3],
    return_btn: Button,
}
impl FoodMenu {
    const MESSAGE: &str = "Select food";

    pub fn new() -> Self {
        Self {
            selected_food: None,
            running: true,
            menu_items: gen_all_food_items(),
            return_btn: Button {
                text: "return".to_string(),
                pos: Vec2::new(
                    (SCREEN_WIDTH as f32 - 50.0) / 2.0,
                    SCREEN_HEIGHT as f32 - 30.0,
                ),
                ..Default::default()
            },
        }
    }

    /// Renders the food selection menu and returns the food selected by the user for feeding.
    /// The selected food can also be `None` when the user exits the food menu via the *'return
    /// button'*.
    pub async fn render(&mut self) -> Option<Food> {
        let msg_dimensions = measure_text(Self::MESSAGE, None, 20, 1.0);

        while self.running {
            clear_background(crate::BACKGROUND_COLOR);

            draw_text(
                Self::MESSAGE,
                (SCREEN_WIDTH as f32 - msg_dimensions.width) / 2.0,
                msg_dimensions.height + 15.0,
                20.0,
                BLACK,
            );

            // Draw the item container
            draw_rectangle(
                ITEM_CONTAINER_AREA.x,
                ITEM_CONTAINER_AREA.y,
                ITEM_CONTAINER_AREA.w,
                ITEM_CONTAINER_AREA.h,
                Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 },
            );

            for item in self.menu_items {
                item.draw();
            }

            self.return_btn.render();
            self.update();

            next_frame().await;
        }

        self.selected_food
    }

    fn update(&mut self) {
        for item in self.menu_items {
            if item.is_clicked() {
                self.selected_food = Some(item.food);
            }
        }

        if self.selected_food.is_some() || self.return_btn.is_clicked() {
            self.running = false;
        }
    }
}

/// Generates and returns all the **FoodMenuItems** that should be displayed in the **FoodMenu**.
/// This function makes sure all items have the correct location on the screen, and that the correct
/// food items are present and in the right order.
fn gen_all_food_items() -> [FoodMenuItem; 3] {
    let mut items = [
        FoodMenuItem::new(Food::Soup),
        FoodMenuItem::new(Food::Cookie),
        FoodMenuItem::new(Food::Burger),
    ];

    let base_height = ITEM_CONTAINER_AREA.y + 10.0;

    for (i, item) in items.iter_mut().enumerate() {
        item.area.y = i as f32 * (FoodMenuItem::DIMENSIONS.height + 5.0) + base_height;
    }

    items
}
