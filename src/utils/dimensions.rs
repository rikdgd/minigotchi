#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

impl From<(f32, f32)> for Dimensions {
    fn from(value: (f32, f32)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}
