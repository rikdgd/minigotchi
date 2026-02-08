use macroquad::prelude::*;
use macroquad::rand::gen_range;
use macroquad::texture::Texture2D;
use serde::{Serialize, Deserialize};

#[macro_export]
macro_rules! include_texture {
    ($sprite_path:expr) => {
        Texture2D::from_file_with_format(include_bytes!($sprite_path), None)
    };
}

const NUM_SHAPES: u8 = 7;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreatureShapes {
    Turtle,
    Snail,
    Fish,
    Mouse,
    Frog,
    Squid,
    Sheep,
}

impl CreatureShapes {
    pub fn get_texture(&self) -> Texture2D {
        match self {
            CreatureShapes::Turtle => include_texture!("../resources/turtle.png"),
            CreatureShapes::Snail => include_texture!("../resources/snail.png"),
            CreatureShapes::Fish => include_texture!("../resources/fish.png"),
            CreatureShapes::Mouse => include_texture!("../resources/mouse.png"),
            CreatureShapes::Frog => include_texture!("../resources/frog.png"),
            CreatureShapes::Squid => include_texture!("../resources/squid.png"),
            CreatureShapes::Sheep => include_texture!("../resources/sheep.png"),
        }
    }
    
    pub fn new_random() -> Self {
        match gen_range(0, NUM_SHAPES) {
            0 => Self::Turtle,
            1 => Self::Snail,
            2 => Self::Fish,
            3 => Self::Mouse,
            4 => Self::Frog,
            5 => Self::Squid,
            _ => Self::Sheep,
        }
    }
}

pub fn egg_shape() -> Texture2D {
    include_texture!("../resources/egg.png")
}

pub fn baby_shape() -> Texture2D {
    include_texture!("../resources/baby.png")
}

pub fn kid_shape() -> Texture2D {
    include_texture!("../resources/kid.png")
}

pub fn sleeping_icon() -> Texture2D {
    include_texture!("../resources/zz.png")
}
