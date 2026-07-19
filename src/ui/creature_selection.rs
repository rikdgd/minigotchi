use macroquad::prelude::*;
use serde::Serialize;
use crate::creature::Creature;
use crate::game_state::GameState;
use crate::shapes::CreatureShapes;
use crate::ui::button::Button;


#[derive(Debug, Clone)]
pub struct CreatureSelection {
    selected_shape: CreatureShapes,
    next_btn: Button,
}

impl CreatureSelection {
    pub fn render(&mut self) {
        self.next_btn.render();

        self.update();
    }

    fn update(&mut self) {
        if self.next_btn.is_clicked() {
            self.selected_shape = Self::next_creature(self.selected_shape);
        }
    }

    fn next_creature(creature: CreatureShapes) -> CreatureShapes {
        match creature {
            CreatureShapes::Turtle => CreatureShapes::Snail,
            CreatureShapes::Snail => CreatureShapes::Fish,
            CreatureShapes::Fish => CreatureShapes::Mouse,
            CreatureShapes::Mouse => CreatureShapes::Frog,
            CreatureShapes::Frog => CreatureShapes::Squid,
            CreatureShapes::Squid => CreatureShapes::Sheep,
            CreatureShapes::Sheep => CreatureShapes::Turtle,
        }
    }
}

impl Default for CreatureSelection {
    fn default() -> Self {
        let mut btn = Button::default();
        btn.text = "->".to_string();
        btn.pos = (100.0, 100.0).into();

        Self {
            selected_shape: CreatureShapes::Turtle,
            next_btn: btn,
        }
    }
}
