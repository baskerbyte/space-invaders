use ruscii::spatial::Direction;

pub trait Offset {
    fn offset(&self) -> i32;
}

impl Offset for Direction {
    fn offset(&self) -> i32 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        }
    }
}