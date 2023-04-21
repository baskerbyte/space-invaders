use std::mem::take;
use std::time::Duration;
use ruscii::drawing::Pencil;
use ruscii::spatial::{Direction, Vec2};
use crate::drawable::Drawable;
use crate::offset::Offset;
use crate::shoot::Shoot;
use crate::timer::Timer;

pub struct Player {
    position: Vec2,
    timer: Timer,
    shoots: Vec<Shoot>
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            timer: Timer::new(Duration::from_millis(40)),
            shoots: Vec::new(),
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

    pub fn shoot(&mut self) {
        if self.shoots.len() < 5 {
            self.shoots.push(Shoot::new(self.position, Direction::Up));
        }
    }

    pub fn draw_shoots(&mut self, pencil: &mut Pencil) {
        for (i, shoot) in self.shoots.iter_mut().enumerate() {
            shoot.draw(pencil);

            if shoot.get_character() == "*" {
                self.shoots.remove(i);
                return;
            }

            if shoot.timer.is_ready() {
                shoot._move();
                shoot.timer.reset();
            }
        }
    }

    fn hmove(&mut self, direction: Direction) {
        if self.timer.is_ready() {
            self.position.x += direction.offset();

            self.timer.reset();
        }
    }
}

impl Drawable for Player {
    fn get_character(&self) -> &str {
        "A"
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }
}
