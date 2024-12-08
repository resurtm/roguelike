use crate::input::Input;

pub struct Player {
    pub position: (f64, f64),

    pub velocity: (f64, f64),
    velocity_delta: f64,
    velocity_max: f64,
    velocity_slowdown: f64,

    pub movement_state: PlayerMovementState,
    pub is_attack: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: (250.0, 350.0),

            velocity: (0.0, 0.0),
            velocity_delta: 0.35,
            velocity_max: 3.5,
            velocity_slowdown: 0.92,

            movement_state: PlayerMovementState::Idle,
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

    pub fn is_walk(&self) -> bool {
        return self.velocity.0.abs() > 0.05 || self.velocity.1.abs() > 0.05;
    }
}

pub enum PlayerMovementState {
    Idle,
    Walk,
    Run,
}
