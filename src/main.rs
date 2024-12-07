mod direction;
mod input;
mod player;

use direction::Direction;
use input::Input;
use player::Player;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
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
