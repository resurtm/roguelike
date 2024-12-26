use crate::{
    consts::{START_POSITION_X, START_POSITION_Y},
    input::Input,
    player::Player,
};
use cgmath::{InnerSpace, MetricSpace, Point2};

pub(crate) struct Camera {
    pub(crate) position: Point2<f64>,
}

impl Camera {
    pub(crate) fn new() -> Camera {
        Camera { position: Point2::new(START_POSITION_X, START_POSITION_Y) }
    }

    pub(crate) fn sync_input(&mut self, input: &Input) {
        if input.key_w {
            self.position.y -= CAMERA_MANUAL_SPEED;
        }
        if input.key_s {
            self.position.y += CAMERA_MANUAL_SPEED;
        }
        if input.key_a {
            self.position.x -= CAMERA_MANUAL_SPEED;
        }
        if input.key_d {
            self.position.x += CAMERA_MANUAL_SPEED;
        }
    }

    pub(crate) fn follow(&mut self, player: &Player) {
        if self.position.distance(player.position) > CAMERA_FOLLOW_THRESHOLD {
            let dir = (player.position - self.position).normalize();
            self.position += dir * CAMERA_FOLLOW_SPEED;
        }
    }
}

const CAMERA_MANUAL_SPEED: f64 = 10.0;
const CAMERA_FOLLOW_SPEED: f64 = 3.5;
const CAMERA_FOLLOW_THRESHOLD: f64 = 350.0;
