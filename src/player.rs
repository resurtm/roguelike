use crate::input::Input;

pub struct Player {
    pub position: (f32, f32),

    pub velocity: (f32, f32),
    velocity_delta: f32,
    velocity_max: f32,
    velocity_slowdown: f32,

    pub is_attack: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: (250.0, 350.0),

            velocity: (0.0, 0.0),
            velocity_delta: 0.35,
            velocity_max: 6.5,
            velocity_slowdown: 0.92,

            is_attack: false,
        }
    }

    pub fn advance(&mut self, input: &Input) {
        // thrust
        if input.key_up {
            self.velocity.1 -= self.velocity_delta
        }
        if input.key_down {
            self.velocity.1 += self.velocity_delta
        }
        if input.key_left {
            self.velocity.0 -= self.velocity_delta
        }
        if input.key_right {
            self.velocity.0 += self.velocity_delta
        }

        // attack
        self.is_attack = input.key_space;

        // update position
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );

        // update velocity
        self.velocity = (
            self.velocity.0 * self.velocity_slowdown,
            self.velocity.1 * self.velocity_slowdown,
        );

        // limit max capacity
        if self.velocity.0 > self.velocity_max {
            self.velocity.0 = self.velocity_max;
        }
        if self.velocity.0 < -self.velocity_max {
            self.velocity.0 = -self.velocity_max;
        }
        if self.velocity.1 > self.velocity_max {
            self.velocity.1 = self.velocity_max;
        }
        if self.velocity.1 < -self.velocity_max {
            self.velocity.1 = -self.velocity_max;
        }
    }
}
