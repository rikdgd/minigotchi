use macroquad::prelude::*;
use macroquad::rand::gen_range;
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
    
    pub fn new_random() -> Self {
        match gen_range(0, NUM_SHAPES) {
            0 => Self::Turtle,
            1 => Self::Snail,
            2 => Self::Fish,
            3 => Self::Mouse,
            4 => Self::Frog,
            _ => Self::Squid,
        }
    }
}

pub fn egg_shape() -> Texture2D {
    Texture2D::from_file_with_format(include_bytes!("../resources/egg.png"), None)
}

pub fn baby_shape() -> Texture2D {
    Texture2D::from_file_with_format(include_bytes!("../resources/baby.png"), None)
}

pub fn kid_shape() -> Texture2D {
    Texture2D::from_file_with_format(include_bytes!("../resources/kid.png"), None)
}

pub fn sleeping_icon() -> Texture2D {
    Texture2D::from_file_with_format(include_bytes!("../resources/zz.png"), None)
}