use crate::{aabb::Aabb, input::Input};
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

        // FIXME: Two testing fictional walls, later implement
        // BFS to detect real walls and use them as AABBs.
        let wall0 = Aabb::new(Point2::new(0.0, 0.0), Point2::new(96.0 * 10.0, 96.0));
        let wall1 = Aabb::new(Point2::new(0.0, 0.0), Point2::new(96.0, 96.0 * 10.0));
        let plb = Aabb::new(
            Point2::new(self.position.x - 96.0 / 4.0, self.position.y - 96.0 / 4.0),
            Point2::new(self.position.x + 96.0 / 4.0, self.position.y + 96.0 / 4.0),
        );

        let cont = wall0.check_contact(&plb);
        if cont.intersects {
            let offset = cont.min_trans * cont.penetration;
            self.position -= Vector2::new(offset.x, offset.y);
        }
        let cont = wall1.check_contact(&plb);
        if cont.intersects {
            let offset = cont.min_trans * cont.penetration;
            self.position -= Vector2::new(offset.x, offset.y);
        }
    }
}
