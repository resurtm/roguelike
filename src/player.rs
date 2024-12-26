use crate::{aabb::Aabb, input::Input, level_collision::LevelCollision};
use cgmath::{Point2, Vector2};

pub(crate) struct Player {
    pub(crate) position: Point2<f64>,

    pub(crate) velocity: Vector2<f64>,
    velocity_delta: f64,
    velocity_max: f64,
    velocity_slowdown: f64,

    pub(crate) is_attack: bool,
}

impl Player {
    pub(crate) fn new() -> Player {
        Player {
            position: Point2::new(450.0, 300.0),

            velocity: Vector2::new(0.0, 0.0),
            velocity_delta: 0.35,
            velocity_max: 6.5,
            velocity_slowdown: 0.92,

            is_attack: false,
        }
    }

    pub(crate) fn sync_input(&mut self, input: &Input) {
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

    pub(crate) fn sync_collision(&mut self, col: &LevelCollision) {
        let p = Aabb::new(
            Point2::new(self.position.x - 96.0 / 4.0, self.position.y - 96.0 / 4.0),
            Point2::new(self.position.x + 96.0 / 4.0, self.position.y + 96.0 / 4.0),
        );

        col.aabbs.iter().for_each(|aabb| {
            let cont = aabb.check_contact(&p);
            if cont.intersects {
                let offset = cont.min_trans * cont.penetration;
                self.position -= Vector2::new(offset.x, offset.y);
            }
        });
    }
}
