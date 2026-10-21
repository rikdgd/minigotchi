use crate::utils::random_helpers::get_random_bool;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum XDirection {
    Left,
    Right,
}
impl XDirection {
    pub fn new_random() -> Self {
        match get_random_bool() {
            true => Self::Left,
            false => Self::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YDirection {
    Up,
    Down,
}
impl YDirection {
    pub fn new_random() -> Self {
        match get_random_bool() {
            true => Self::Up,
            false => Self::Down,
        }
    }
}
