use serde::{Serialize, Deserialize};
use crate::items::creature_color::CreatureColor;


#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub creature_colors: Vec<CreatureColor>,
}
