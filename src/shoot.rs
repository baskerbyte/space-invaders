use std::time::Duration;
use ruscii::spatial::{Direction, Vec2};
use crate::drawable::Drawable;
use crate::offset::Offset;
use crate::timer::Timer;

pub struct Shoot {
    pub timer: Timer,
    pub direction: Direction,
    position: Vec2,
}

impl Shoot {
    pub fn new(position: Vec2, direction: Direction) -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(45)),
            direction,
            position: Vec2::xy(position.x, position.y + direction.offset()),
        }
    }

    pub fn _move(&mut self) {
        self.position.y += self.direction.offset();
    }
}

impl Drawable for Shoot {
    fn get_character(&self) -> &str {
        if self.position.y <= 2 || self.position.y >= 18 {
            "*"
        } else {
            "|"
        }
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }
}
