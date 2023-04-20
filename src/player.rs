use std::time::Duration;
use ruscii::spatial::{Direction, Vec2};
use crate::offset::Offset;
use crate::timer::Timer;

pub struct Player {
    pub position: Vec2,
    timer: Timer,
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            timer: Timer::new(Duration::from_millis(40))
        }
    }

    pub fn move_right(&mut self) {
        self.hmove(Direction::Right);
    }

    pub fn move_left(&mut self) {
        self.hmove(Direction::Left);
    }

    fn hmove(&mut self, direction: Direction) {
        if self.timer.is_ready() {
            self.position.x += direction.offset();

            self.timer.reset();
        }
    }
}
