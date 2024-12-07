use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread::sleep;
use std::time::Duration;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Player {
    position: (f64, f64),

    speed: (f64, f64),
    speed_delta: f64,
    speed_max: f64,
    speed_slowdown: f64,

    size: f64,
}

impl Player {
    fn new() -> Player {
        Player {
            position: (250.0, 350.0),

            speed: (0.0, 0.0),
            speed_delta: 0.15,
            speed_max: 2.5,
            speed_slowdown: 0.92,

            size: 25.0,
        }
    }

    fn thrust(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.speed.1 -= self.speed_delta,
            Direction::Down => self.speed.1 += self.speed_delta,
            Direction::Left => self.speed.0 -= self.speed_delta,
            Direction::Right => self.speed.0 += self.speed_delta,
        }
        self.cap_max();
    }

    fn advance(&mut self) {
        self.position = (
            self.position.0 + self.speed.0,
            self.position.1 + self.speed.1,
        );
        self.speed = (
            self.speed.0 * self.speed_slowdown,
            self.speed.1 * self.speed_slowdown,
        );
        self.cap_max();
    }

    fn cap_max(&mut self) {
        if self.speed.0 > self.speed_max {
            self.speed.0 = self.speed_max;
        }
        if self.speed.0 < -self.speed_max {
            self.speed.0 = -self.speed_max;
        }
        if self.speed.1 > self.speed_max {
            self.speed.1 = self.speed_max;
        }
        if self.speed.1 < -self.speed_max {
            self.speed.1 = -self.speed_max;
        }
    }
}

struct Input {
    key_up: bool,
    key_down: bool,
    key_left: bool,
    key_right: bool,
}

impl Input {
    fn new() -> Input {
        Input {
            key_up: false,
            key_down: false,
            key_left: false,
            key_right: false,
        }
    }

    fn handle_key_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(k), ..
            } => match *k {
                Keycode::UP => self.key_up = true,
                Keycode::DOWN => self.key_down = true,
                Keycode::LEFT => self.key_left = true,
                Keycode::RIGHT => self.key_right = true,
                _ => {}
            },
            Event::KeyUp {
                keycode: Some(k), ..
            } => match *k {
                Keycode::UP => self.key_up = false,
                Keycode::DOWN => self.key_down = false,
                Keycode::LEFT => self.key_left = false,
                Keycode::RIGHT => self.key_right = false,
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rogue like", 1024, 768)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("./assets/red-tile.png")
        .unwrap();

    let mut player = Player::new();
    let mut input = Input::new();

    'running: loop {
        // events and input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { .. } | Event::KeyUp { .. } => {
                    input.handle_key_event(&event);
                }
                _ => {}
            }
        }

        // calculate
        if input.key_up {
            player.thrust(Direction::Up);
        }
        if input.key_down {
            player.thrust(Direction::Down);
        }
        if input.key_left {
            player.thrust(Direction::Left);
        }
        if input.key_right {
            player.thrust(Direction::Right);
        }
        player.advance();

        // render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let rect = Rect::new(
            (player.position.0 - player.size) as i32,
            (player.position.1 - player.size) as i32,
            (player.size * 2.0) as u32,
            (player.size * 2.0) as u32,
        );
        canvas.copy(&texture, None, rect).unwrap();

        // present and sleep
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
