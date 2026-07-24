use crate::ui::button::Button;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};


pub fn shop_button() -> Button {
    const BTN_DIMENSION: f32 = 10.0;
    let x_pos = SCREEN_WIDTH as f32 - BTN_DIMENSION * 2.0;
    let y_pos = (SCREEN_HEIGHT / 2) as f32 - 6.0;
    
    Button {
        pos: (x_pos, y_pos).into(),
        size: (BTN_DIMENSION, BTN_DIMENSION).into(),
        text: "$".to_string(),
        fontsize: 16.0,
        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ShopPage {
    
}