use std::time::{Duration, Instant};

pub struct Timer {
    delay: Duration,
    instant: Instant
}

impl Timer {
    pub fn new(delay: Duration) -> Self {
        Self {
            delay,
            instant: Instant::now()
        }
    }

    pub fn is_ready(&self) -> bool {
        self.instant.elapsed() >= self.delay
    }

    pub fn reset(&mut self) {
        self.instant = Instant::now()
    }
}
