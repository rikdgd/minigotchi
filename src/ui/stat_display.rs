use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use crate::friend::Friend;

const RENDER_COLOR: Color = Color::new(0.35, 0.35, 0.35, 1.0);

/// Renders a visualisation of the creature's status, using status bars. 
/// 
/// ## Parameters:
/// * `stat` - The stat which should be displayed by the status bar.
/// * `icon` - The icon that will be drawn in front of the status bar.
/// * `y_location` - The y location where the status bar should be rendered on screen.
pub fn stat_display(stat: &Friend, icon: StatIcon) {
    draw_texture(&StatIcon::Food.get_texture(), 0.0, 100.0, RENDER_COLOR);
    draw_rectangle(10.0, 100.0, 180.0, 15.0, RENDER_COLOR);
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
            StatIcon::Food => Texture2D::from_file_with_format(include_bytes!("../../resources/status_icons/heart.png"), None),
            StatIcon::Joy => Texture2D::from_file_with_format(include_bytes!("../../resources/status_icons/heart.png"), None),
            StatIcon::Energy => Texture2D::from_file_with_format(include_bytes!("../../resources/status_icons/heart.png"), None),
            StatIcon::Health => Texture2D::from_file_with_format(include_bytes!("../../resources/status_icons/heart.png"), None),
        }
    }
}
