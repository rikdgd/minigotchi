use macroquad::prelude::*;


pub struct Button {
    pub pos: Vec2,
    pub size: Vec2,
    pub text: String,
    pub color: Color,
}

impl Button {
    pub fn render(&self, mouse_pos: Vec2) {
        let color = if self.collision_rect().contains(mouse_pos) {
            Color::new(
                self.color.r - 0.06,
                self.color.g - 0.06,
                self.color.b - 0.06,
                self.color.a,
            )
        } else {
            self.color
        };

        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, color);

        // Calculates the dimensions of the text so they can be used to position the button text
        let text_size = measure_text(&self.text, None, 12, 1.0);

        let text_x = self.pos.x + (self.size.x - text_size.width) / 2.0;
        let text_y = self.pos.y + (self.size.y + text_size.height) / 2.0;

        draw_text(
            &self.text,
            text_x,
            text_y,
            12.0,
            BLACK,
        );
    }

    pub fn collision_rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y)
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            size: Vec2::new(50.0, 20.0),
            text: String::from("[button]"),
            color: Color::new(0.70, 0.70, 0.70, 1.0),
        }
    }
}
