use crate::input::Input;
use cgmath::{Point2, Vector2};

pub(crate) struct Player {
    pub(crate) position: Point2<f32>,

    pub(crate) velocity: Vector2<f32>,
    velocity_delta: f32,
    velocity_max: f32,
    velocity_slowdown: f32,

    pub(crate) is_attack: bool,
}

impl Player {
    pub(crate) fn new() -> Player {
        Player {
            position: Point2::new(250.0, 250.0),

            velocity: Vector2::new(0.0, 0.0),
            velocity_delta: 0.35,
            velocity_max: 6.5,
            velocity_slowdown: 0.92,

            is_attack: false,
        }
    }

    pub(crate) fn advance(&mut self, input: &Input) {
        // thrust
        if input.key_up {
            self.velocity.y -= self.velocity_delta
        }
        if input.key_down {
            self.velocity.y += self.velocity_delta
        }
        if input.key_left {
            self.velocity.x -= self.velocity_delta
        }
        if input.key_right {
            self.velocity.x += self.velocity_delta
        }

        self.is_attack = input.key_space;
        self.position += self.velocity;
        self.velocity *= self.velocity_slowdown;

        // limit max capacity
        if self.velocity.x > self.velocity_max {
            self.velocity.x = self.velocity_max;
        }
        if self.velocity.x < -self.velocity_max {
            self.velocity.x = -self.velocity_max;
        }
        if self.velocity.y > self.velocity_max {
            self.velocity.y = self.velocity_max;
        }
        if self.velocity.y < -self.velocity_max {
            self.velocity.y = -self.velocity_max;
        }
    }
}
