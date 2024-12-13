use sdl2::{event::Event, keyboard::Keycode};

pub(crate) struct Input {
    pub(crate) key_up: bool,
    pub(crate) key_down: bool,
    pub(crate) key_left: bool,
    pub(crate) key_right: bool,

    pub(crate) key_w: bool,
    pub(crate) key_s: bool,
    pub(crate) key_a: bool,
    pub(crate) key_d: bool,

    pub(crate) key_space: bool,
}

impl Input {
    pub(crate) fn new() -> Input {
        Input {
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

    pub(crate) fn handle_key_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(k), ..
            } => match *k {
                Keycode::UP => self.key_up = true,
                Keycode::DOWN => self.key_down = true,
                Keycode::LEFT => self.key_left = true,
                Keycode::RIGHT => self.key_right = true,
                Keycode::W => self.key_w = true,
                Keycode::S => self.key_s = true,
                Keycode::A => self.key_a = true,
                Keycode::D => self.key_d = true,
                Keycode::SPACE => self.key_space = true,
                _ => {}
            },
            Event::KeyUp {
                keycode: Some(k), ..
            } => match *k {
                Keycode::UP => self.key_up = false,
                Keycode::DOWN => self.key_down = false,
                Keycode::LEFT => self.key_left = false,
                Keycode::RIGHT => self.key_right = false,
                Keycode::W => self.key_w = false,
                Keycode::S => self.key_s = false,
                Keycode::A => self.key_a = false,
                Keycode::D => self.key_d = false,
                Keycode::SPACE => self.key_space = false,
                _ => {}
            },
            _ => {}
        }
    }
}
