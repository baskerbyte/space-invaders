use std::ops::Range;
use std::time::Duration;
use ruscii::drawing::Pencil;
use ruscii::spatial::Vec2;
use crate::alien::Alien;
use crate::drawable::Drawable;
use crate::timer::Timer;

pub struct Army {
    pub aliens: Vec<Alien>,
    timer: Timer,
    direction: i32
}

impl Army {
    pub fn new(range: Range<Vec2>) -> Self {
        Self {
            aliens: populate(range),
            timer: Timer::new(Duration::from_millis(250)),
            direction: 1,
        }
    }

    pub fn draw_aliens(&mut self, pencil: &mut Pencil) {
        for alien in self.aliens.iter_mut() {
            alien.draw(pencil);
        }

        if self.timer.is_ready() {
            self.move_aliens();
            self.timer.reset();
        }
    }

    fn move_aliens(&mut self) {
        let border_reached = self.aliens.iter().any(|alien| {
            alien.get_position().x == 1 || alien.get_position().x == 38
        });

        if border_reached {
            self.direction *= -1;
        }

        for alien in self.aliens.iter_mut() {
            if border_reached {
                alien.move_down();
            }

            alien.position.x += self.direction;
        }
    }

    pub fn dead(&self) -> bool {
        self.aliens.len() == 0 || self.aliens.iter().any(|alien| alien.get_position().y > 17)
    }
}

fn populate(range: Range<Vec2>) -> Vec<Alien> {
    let mut aliens = Vec::new();

    for x in range.start.x..range.end.x {
        for y in range.start.y..range.end.y {
            if x % 2 == 0 && y % 2 == 0 {
                aliens.push(Alien::new(Vec2::xy(x, y)));
            }
        }
    };

    aliens
}
