use sdl2::event::Event;
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
            speed_delta: 0.1,
            speed_max: 3.5,
            speed_slowdown: 0.95,

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

    let mut player = Player::new();

    'running: loop {
        // events and input
        for event in event_pump.poll_iter() {
            println!("{:?}", event);
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => player.thrust(Direction::Up),
                Event::KeyDown {
                    keycode: Some(Keycode::DOWN),
                    ..
                } => player.thrust(Direction::Down),
                Event::KeyDown {
                    keycode: Some(Keycode::LEFT),
                    ..
                } => player.thrust(Direction::Left),
                Event::KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    ..
                } => player.thrust(Direction::Right),
                _ => {}
            }
        }

        // calculate
        player.advance();

        // render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas
            .draw_rect(Rect::new(
                (player.position.0 - player.size) as i32,
                (player.position.1 - player.size) as i32,
                (player.size * 2.0) as u32,
                (player.size * 2.0) as u32,
            ))
            .unwrap();

        // present and sleep
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
