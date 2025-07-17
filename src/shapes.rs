use macroquad::texture::Texture2D;
use serde::{Serialize, Deserialize};

const NUM_SHAPES: u32 = 6;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreatureShapes {
    Turtle,
    Snail,
    Fish,
    Mouse,
    Frog,
    Squid,
}

impl CreatureShapes {
    pub fn get_texture(&self) -> Texture2D {
        match self {
            CreatureShapes::Turtle => Texture2D::from_file_with_format(include_bytes!("../resources/turtle.png"), None),
            CreatureShapes::Snail => Texture2D::from_file_with_format(include_bytes!("../resources/snail.png"), None),
            CreatureShapes::Fish => Texture2D::from_file_with_format(include_bytes!("../resources/fish.png"), None),
            CreatureShapes::Mouse => Texture2D::from_file_with_format(include_bytes!("../resources/mouse.png"), None),
            CreatureShapes::Frog => Texture2D::from_file_with_format(include_bytes!("../resources/frog.png"), None),
            CreatureShapes::Squid => Texture2D::from_file_with_format(include_bytes!("../resources/squid.png"), None),
        }
    }
}
