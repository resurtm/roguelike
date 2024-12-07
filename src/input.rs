use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
    pub key_up: bool,
    pub key_down: bool,
    pub key_left: bool,
    pub key_right: bool,
    pub key_space: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            key_up: false,
            key_down: false,
            key_left: false,
            key_right: false,
            key_space: false,
        }
    }

    pub fn handle_key_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(k), ..
            } => match *k {
                Keycode::UP => self.key_up = true,
                Keycode::DOWN => self.key_down = true,
                Keycode::LEFT => self.key_left = true,
                Keycode::RIGHT => self.key_right = true,
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
                Keycode::SPACE => self.key_space = false,
                _ => {}
            },
            _ => {}
        }
    }
}
