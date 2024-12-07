use crate::input::Input;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};
use std::{thread::sleep, time::Duration};

pub struct DirectMedia {
    pub is_main_loop_active: bool,

    event_pump: EventPump,

    pub canvas: Canvas<Window>,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl DirectMedia {
    pub fn new() -> DirectMedia {
        let sdl_context = sdl2::init().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("roguelike", 1920, 1200)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        DirectMedia {
            is_main_loop_active: true,

            event_pump,
            canvas,

            texture_creator,
        }
    }

    pub fn handle_events(&mut self, input: &mut Input) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_main_loop_active = false,
                Event::KeyUp { .. } | Event::KeyDown { .. } => input.handle_key_event(&event),
                _ => {}
            }
        }
    }

    pub fn present_start(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present_end(&mut self) {
        self.canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
