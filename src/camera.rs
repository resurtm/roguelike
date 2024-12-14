use crate::{input::Input, player::Player};
use cgmath::{InnerSpace, MetricSpace, Point2};

pub(crate) struct Camera {
    pub(crate) position: Point2<f32>,
}

impl Camera {
    pub(crate) fn new() -> Camera {
        Camera {
            position: Point2::new(450.0, 300.0),
        }
    }

    pub(crate) fn sync(&mut self, input: &Input) {
        if input.key_w {
            self.position.y -= 10.0;
        }
        if input.key_s {
            self.position.y += 10.0;
        }
        if input.key_a {
            self.position.x -= 10.0;
        }
        if input.key_d {
            self.position.x += 10.0;
        }
    }

    pub(crate) fn follow(&mut self, player: &Player) {
        if self.position.distance(player.position) > 400.0 {
            let dir = (player.position - self.position).normalize();
            self.position += dir * 3.5;
        }
    }
}
