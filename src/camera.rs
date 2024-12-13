use crate::input::Input;
use cgmath::Point2;

pub(crate) struct Camera {
    pub(crate) pos: Point2<f32>,
}

impl Camera {
    pub(crate) fn new() -> Camera {
        Camera {
            pos: Point2::new(0.0, 0.0),
        }
    }
    pub(crate) fn sync(&mut self, input: &Input) {
        if input.key_w {
            self.pos.y -= 15.0;
        }
        if input.key_s {
            self.pos.y += 15.0;
        }
        if input.key_a {
            self.pos.x -= 15.0;
        }
        if input.key_d {
            self.pos.x += 15.0;
        }
    }
}
