use macroquad::prelude::*;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ui::button::Button;
use crate::ui::interaction_menu::{InteractionMenuItem, ITEM_CONTAINER_AREA, CreatureInteraction};


/// The **InteractionMenu** struct is used to render a UI used to perform an interaction with the
/// creature.
/// 
/// ## Fields:
/// * `selected_interaction` - The interaction the user has selected to perform on their creature.
/// * `running` - The running field is used to keep the render loop running. Setting it to false
///   stops the render loop. `true` by default.
/// * `menu_items` - These are the *button* UI elements the user can click to select an interaction.
/// * `return_btn` - A button UI component used to exit the **InteractionMenu**.
#[derive(Debug, Clone)]
pub struct InteractionMenu<T>
where
    T: CreatureInteraction
{
    selected_interaction: Option<T>,
    running: bool,
    menu_items: [InteractionMenuItem<T>; 3],
    return_btn: Button,
}
impl<T> InteractionMenu<T> 
where
    T: CreatureInteraction
{
    pub fn new(menu_items: [InteractionMenuItem<T>; 3]) -> Self {
        Self {
            selected_interaction: None,
            running: true,
            menu_items,
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
    pub async fn render(&mut self) -> Option<T> {
        let msg_dimensions = measure_text(&T::menu_title(), None, 20, 1.0);

        while self.running {
            clear_background(crate::BACKGROUND_COLOR);

            draw_text(
                &T::menu_title(),
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

        self.selected_interaction
    }

    fn update(&mut self) {
        for item in self.menu_items {
            if item.is_clicked() {
                self.selected_interaction = Some(item.interaction);
            }
        }

        if self.selected_interaction.is_some() || self.return_btn.is_clicked() {
            self.running = false;
        }
    }
}


