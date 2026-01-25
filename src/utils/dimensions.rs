#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}
impl Dimensions {
    pub fn surface(&self) -> f32 {
        self.width * self.height
    }
}