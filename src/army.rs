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
}

impl Army {
    pub fn new(range: Range<Vec2>) -> Self {
        Self {
            aliens: populate(range),
            timer: Timer::new(Duration::from_millis(500))
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

    pub fn get_alien_at_position(&self, position: Vec2) -> Option<(usize, &Alien)> {
        self.aliens.iter().enumerate().find(|(_i, alien)| {
            alien.get_position() == position
        })
    }

    pub fn remove_alien_at(&mut self, position: Vec2) -> bool {
        if let Some((i, _)) = self.get_alien_at_position(position) {
            self.aliens.remove(i);
            return true;
        }

        false
    }

    fn move_aliens(&mut self) {
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
