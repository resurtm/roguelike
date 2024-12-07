use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread::sleep;
use std::time::Duration;

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

    let size = 20f64;
    let velocity = 15f64;
    let mut pos = (200f64, 300f64);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => pos.1 -= velocity,
                Event::KeyDown {
                    keycode: Some(Keycode::DOWN),
                    ..
                } => pos.1 += velocity,
                Event::KeyDown {
                    keycode: Some(Keycode::LEFT),
                    ..
                } => pos.0 -= velocity,
                Event::KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    ..
                } => pos.0 += velocity,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas
            .draw_rect(Rect::new(
                (pos.0 - size) as i32,
                (pos.1 - size) as i32,
                (size * 2.0) as u32,
                (size * 2.0) as u32,
            ))
            .unwrap();

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
