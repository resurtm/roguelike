use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey::Code};

pub struct Input {
    pub key_up: bool,
    pub key_down: bool,
    pub key_left: bool,
    pub key_right: bool,

    pub key_w: bool,
    pub key_s: bool,
    pub key_a: bool,
    pub key_d: bool,

    pub key_space: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            key_up: false,
            key_down: false,
            key_left: false,
            key_right: false,

            key_w: false,
            key_s: false,
            key_a: false,
            key_d: false,

            key_space: false,
        }
    }

    pub fn handle_key_event(&mut self, e: &KeyEvent) {
        if let Code(key_code) = e.physical_key {
            let t = e.state == ElementState::Pressed;
            match key_code {
                KeyCode::ArrowUp => self.key_up = t,
                KeyCode::ArrowDown => self.key_down = t,
                KeyCode::ArrowLeft => self.key_left = t,
                KeyCode::ArrowRight => self.key_right = t,

                KeyCode::KeyW => self.key_w = t,
                KeyCode::KeyS => self.key_s = t,
                KeyCode::KeyA => self.key_a = t,
                KeyCode::KeyD => self.key_d = t,

                KeyCode::Space => self.key_space = t,

                _ => {}
            }
        }
    }
}
