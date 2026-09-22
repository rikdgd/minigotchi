use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use serde::{Serialize, Deserialize};
use crate::utils::Dimensions;

#[macro_export]
macro_rules! include_texture {
    ($sprite_path:expr) => {
        macroquad::texture::Texture2D::from_file_with_format(include_bytes!($sprite_path), None)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CreatureShape {
    Turtle,
    Snail,
    Fish,
    Mouse,
    Frog,
    Squid,
    Sheep,
    Germ,
    Jellyfish,
    Chicken,
    Ufo,
    Bunny,
    Bug,
    Crab,
    AppleBoy,
    Furby,
    Bee,
    Flowey,
    Cultist,
    Sapling,
}

impl CreatureShape {
    /// An array containing all possible `CreatureShapes` variants.
    pub const ALL_VARIANTS: [Self; 20] = [
        CreatureShape::Turtle,
        CreatureShape::Snail,
        CreatureShape::Fish,
        CreatureShape::Mouse,
        CreatureShape::Frog,
        CreatureShape::Squid,
        CreatureShape::Sheep,
        CreatureShape::Germ,
        CreatureShape::Jellyfish,
        CreatureShape::Chicken,
        CreatureShape::Ufo,
        CreatureShape::Bunny,
        CreatureShape::Bug,
        CreatureShape::Crab,
        CreatureShape::AppleBoy,
        CreatureShape::Furby,
        CreatureShape::Bee,
        CreatureShape::Flowey,
        CreatureShape::Cultist,
        CreatureShape::Sapling,
    ];
    
    pub const TEXTURE_DIMENSIONS: Dimensions = Dimensions { width: 25., height: 25. };
    
    pub fn get_texture(&self) -> Texture2D {
        match self {
            CreatureShape::Turtle => include_texture!("../resources/turtle.png"),
            CreatureShape::Snail => include_texture!("../resources/snail.png"),
            CreatureShape::Fish => include_texture!("../resources/fish.png"),
            CreatureShape::Mouse => include_texture!("../resources/mouse.png"),
            CreatureShape::Frog => include_texture!("../resources/frog.png"),
            CreatureShape::Squid => include_texture!("../resources/squid.png"),
            CreatureShape::Sheep => include_texture!("../resources/sheep.png"),
            CreatureShape::Germ => include_texture!("../resources/germ.png"),
            CreatureShape::Jellyfish => include_texture!("../resources/jellyfish.png"),
            CreatureShape::Chicken => include_texture!("../resources/chicken.png"),
            CreatureShape::Ufo => include_texture!("../resources/ufo.png"),
            CreatureShape::Bunny => include_texture!("../resources/bunny.png"),
            CreatureShape::Bug => include_texture!("../resources/bug.png"),
            CreatureShape::Crab => include_texture!("../resources/crab.png"),
            CreatureShape::AppleBoy => include_texture!("../resources/appleboy.png"),
            CreatureShape::Furby => include_texture!("../resources/furby.png"),
            CreatureShape::Bee => include_texture!("../resources/bee.png"),
            CreatureShape::Flowey => include_texture!("../resources/flowey.png"),
            CreatureShape::Cultist => include_texture!("../resources/cultist.png"),
            CreatureShape::Sapling => include_texture!("../resources/sapling.png"),
        }
    }
    
    /// Returns the sprite that can be used as a *'backdrop'* for the given creature shape. The
    /// backdrop can be drawn behind the texture received from the `CreatureShapes::get_texture` method
    /// to prevent any background pixels being visible through the creature's sprite.
    pub fn get_backdrop(&self) -> Texture2D {
        match self {
            CreatureShape::Turtle => include_texture!("../resources/turtle_backdrop.png"),
            CreatureShape::Snail => include_texture!("../resources/snail_backdrop.png"),
            CreatureShape::Fish => include_texture!("../resources/fish_backdrop.png"),
            CreatureShape::Mouse => include_texture!("../resources/mouse_backdrop.png"),
            CreatureShape::Frog => include_texture!("../resources/frog_backdrop.png"),
            CreatureShape::Squid => include_texture!("../resources/squid_backdrop.png"),
            CreatureShape::Sheep => include_texture!("../resources/sheep_backdrop.png"),
            CreatureShape::Germ => include_texture!("../resources/germ_backdrop.png"),
            CreatureShape::Jellyfish => include_texture!("../resources/jellyfish_backdrop.png"),
            CreatureShape::Chicken => include_texture!("../resources/chicken_backdrop.png"),
            CreatureShape::Ufo => include_texture!("../resources/ufo_backdrop.png"),
            CreatureShape::Bunny => include_texture!("../resources/bunny_backdrop.png"),
            CreatureShape::Bug => include_texture!("../resources/bug_backdrop.png"),
            CreatureShape::Crab => include_texture!("../resources/crab_backdrop.png"),
            CreatureShape::AppleBoy => include_texture!("../resources/appleboy_backdrop.png"),
            CreatureShape::Furby => include_texture!("../resources/furby_backdrop.png"),
            CreatureShape::Bee => include_texture!("../resources/bee_backdrop.png"),
            CreatureShape::Flowey => include_texture!("../resources/flowey_backdrop.png"),
            CreatureShape::Cultist => include_texture!("../resources/cultist_backdrop.png"),
            CreatureShape::Sapling => include_texture!("../resources/sapling_backdrop.png"),
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
