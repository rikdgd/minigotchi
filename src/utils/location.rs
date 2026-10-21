/// Represents a Location on the screen of the **minigotchi** application. 
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}
impl Location {
    /// Consumes `self` and returns a new translated (moved) Location.
    /// 
    /// ## Parameters:
    /// * `x` - The translation on the x-axis
    /// * `y` - The translation on the y-axis
    pub fn translate(self, x: f32, y: f32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl From<(f32, f32)> for Location {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
