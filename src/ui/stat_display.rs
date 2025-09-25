use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use crate::creature::Creature;
use crate::utils::Stat;

const Y_SPACING: f32 = 17.0;
const RENDER_COLOR: Color = Color::new(0.35, 0.35, 0.35, 1.0);

/// Renders a visualisation of the creature's status, using status bars. 
/// 
/// ## Parameters:
/// * `stat` - The stat which should be displayed by the status bar.
/// * `icon` - The icon that will be drawn in front of the status bar.
/// * `y_location` - The y location where the status bar should be rendered on screen.
pub fn stat_display(creature: &Creature) {
    let icon_x: f32 = 10.0;
    let rectangle_x: f32 = 30.0;
    
    let mut y_pos: f32 = 110.0;

    let bar_length = |stat: Stat| {
        let value = stat.value() as f32;
        (150.0 / 100.0) * value
    };
    
    // food
    draw_texture(&StatIcon::Food.get_texture(), icon_x, y_pos, RENDER_COLOR);
    draw_rectangle(rectangle_x, y_pos, bar_length(creature.food()), 10.0, RENDER_COLOR);

    // joy
    y_pos += Y_SPACING;
    draw_texture(&StatIcon::Joy.get_texture(), icon_x, y_pos, RENDER_COLOR);
    draw_rectangle(rectangle_x, y_pos, bar_length(creature.joy()), 10.0, RENDER_COLOR);
    
    // energy
    y_pos += Y_SPACING;
    draw_texture(&StatIcon::Energy.get_texture(), icon_x, y_pos, RENDER_COLOR);
    draw_rectangle(rectangle_x, y_pos, bar_length(creature.energy()), 10.0, RENDER_COLOR);

    // health
    y_pos += Y_SPACING;
    draw_texture(&StatIcon::Health.get_texture(), icon_x, y_pos, RENDER_COLOR);
    draw_rectangle(rectangle_x, y_pos, bar_length(creature.health()), 10.0, RENDER_COLOR);
}

#[derive(Debug, Clone, Copy)]
pub enum StatIcon {
    Food,
    Joy,
    Energy,
    Health,
}

impl StatIcon {
    pub fn get_texture(&self) -> Texture2D {
        match self {
            StatIcon::Food => Texture2D::from_file_with_format(include_bytes!("../../resources/status_icons/food.png"), None),
            StatIcon::Joy => Texture2D::from_file_with_format(include_bytes!("../../resources/status_icons/joy.png"), None),
            StatIcon::Energy => Texture2D::from_file_with_format(include_bytes!("../../resources/status_icons/energy.png"), None),
            StatIcon::Health => Texture2D::from_file_with_format(include_bytes!("../../resources/status_icons/heart.png"), None),
        }
    }
}
