use crate::direction::Direction;

pub struct Player {
    pub position: (f64, f64),
    pub size: f64,

    speed: (f64, f64),
    speed_delta: f64,
    speed_max: f64,
    speed_slowdown: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: (250.0, 350.0),
            size: 25.0,

            speed: (0.0, 0.0),
            speed_delta: 0.15,
            speed_max: 2.5,
            speed_slowdown: 0.92,
        }
    }

    pub fn thrust(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.speed.1 -= self.speed_delta,
            Direction::Down => self.speed.1 += self.speed_delta,
            Direction::Left => self.speed.0 -= self.speed_delta,
            Direction::Right => self.speed.0 += self.speed_delta,
        }
        self.cap_max();
    }

    pub fn advance(&mut self) {
        self.position = (
            self.position.0 + self.speed.0,
            self.position.1 + self.speed.1,
        );
        self.speed = (
            self.speed.0 * self.speed_slowdown,
            self.speed.1 * self.speed_slowdown,
        );
        self.cap_max();
    }

    fn cap_max(&mut self) {
        if self.speed.0 > self.speed_max {
            self.speed.0 = self.speed_max;
        }
        if self.speed.0 < -self.speed_max {
            self.speed.0 = -self.speed_max;
        }
        if self.speed.1 > self.speed_max {
            self.speed.1 = self.speed_max;
        }
        if self.speed.1 < -self.speed_max {
            self.speed.1 = -self.speed_max;
        }
    }
}
