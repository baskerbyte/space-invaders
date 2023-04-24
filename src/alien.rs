use ruscii::spatial::{Direction, Vec2};
use crate::drawable::Drawable;
use crate::offset::Offset;

pub struct Alien {
    position: Vec2,
}

impl Alien {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
        }
    }

    pub fn move_right(&mut self) {
        if self.position.x > 37 { return; }

        self.hmove(Direction::Right);
    }

    pub fn move_left(&mut self) {
        if self.position.x < 2 { return; }

        self.hmove(Direction::Left);
    }

    pub fn move_down(&mut self) {
        self.position.y += 1
    }

    fn hmove(&mut self, direction: Direction) {
        self.position.x += direction.offset();
    }
}

impl Drawable for Alien {
    fn get_character(&self) -> &str {
        if self.position.x % 2 == 0 { "X" } else { "*" }
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }
}
