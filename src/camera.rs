use crate::{consts::START_POSITION, input::Input, player::PlayerOld};
use cgmath::{InnerSpace, MetricSpace, Point2};

pub(crate) struct Camera {
    pub(crate) position: Point2<f64>,
}

impl Camera {
    pub(crate) fn new() -> Self {
        Self { position: Point2::new(START_POSITION.0, START_POSITION.1) }
    }

    pub(crate) fn apply_input(&mut self, input: &Input) {
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

    pub(crate) fn follow_player(&mut self, player: &PlayerOld) {
        if self.position.distance(player.position) > CAMERA_FOLLOW_THRESHOLD {
            let dir = (player.position - self.position).normalize();
            self.position += dir * CAMERA_FOLLOW_SPEED;
        }
    }
}

const CAMERA_MANUAL_SPEED: f64 = 10.0;
const CAMERA_FOLLOW_SPEED: f64 = 3.5;
const CAMERA_FOLLOW_THRESHOLD: f64 = 325.0;

#[cfg(test)]
mod tests {
    use super::Camera;
    use crate::{
        camera::{CAMERA_FOLLOW_THRESHOLD, CAMERA_MANUAL_SPEED},
        consts::START_POSITION,
        input::Input,
        player::PlayerOld,
    };
    use cgmath::{assert_relative_eq, ElementWise, Point2};

    #[test]
    fn test_apply_input_up_left() {
        let start_position = Point2::new(START_POSITION.0, START_POSITION.1);
        let mut camera = Camera::new();
        assert_relative_eq!(camera.position, start_position);

        let mut input = Input::new();
        input.key_w = true;
        input.key_s = false;
        input.key_a = true;
        input.key_d = false;

        camera.apply_input(&input);
        assert_relative_eq!(camera.position, start_position.sub_element_wise(CAMERA_MANUAL_SPEED));
    }

    #[test]
    fn test_apply_input_down_right() {
        let start_position = Point2::new(START_POSITION.0, START_POSITION.1);
        let mut camera = Camera::new();
        assert_relative_eq!(camera.position, start_position);

        let mut input = Input::new();
        input.key_w = false;
        input.key_s = true;
        input.key_a = false;
        input.key_d = true;

        camera.apply_input(&input);
        assert_relative_eq!(camera.position, start_position.add_element_wise(CAMERA_MANUAL_SPEED));
    }

    #[test]
    fn test_follow_player_far() {
        let start_position = Point2::new(START_POSITION.0, START_POSITION.1);
        let mut camera = Camera::new();
        assert_relative_eq!(camera.position, start_position);

        let mut player = PlayerOld::new();
        player.position = start_position.add_element_wise(CAMERA_FOLLOW_THRESHOLD * 1.5);

        camera.follow_player(&player);
        assert_relative_eq!(
            camera.position,
            Point2::new(452.474_874, 302.474_874),
            epsilon = 0.000_001
        );
    }

    #[test]
    fn test_follow_player_close() {
        let start_position = Point2::new(START_POSITION.0, START_POSITION.1);
        let mut camera = Camera::new();
        assert_relative_eq!(camera.position, start_position);

        let mut player = PlayerOld::new();
        player.position = start_position.add_element_wise(CAMERA_FOLLOW_THRESHOLD * 0.5);

        camera.follow_player(&player);
        assert_relative_eq!(camera.position, start_position);
    }
}
