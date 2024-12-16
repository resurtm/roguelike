use crate::{
    consts::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH},
    input::Input,
};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    render::{Canvas, TextureCreator},
    video::{Window, WindowBuildError, WindowContext},
    EventPump, IntegerOrSdlError,
};
use std::{thread::sleep, time::Duration};
use thiserror::Error;

pub(crate) struct DirectMedia {
    event_pump: EventPump,
    pub(crate) canvas: Canvas<Window>,
    pub(crate) texture_creator: TextureCreator<WindowContext>,
    is_alive: bool,
}

impl DirectMedia {
    pub(crate) fn new() -> Result<DirectMedia, DirectMediaError> {
        let sdl_context = sdl2::init().map_err(DirectMediaError::Context)?;
        let event_pump = sdl_context.event_pump().map_err(DirectMediaError::EventPump)?;
        let video_subsystem = sdl_context.video().map_err(DirectMediaError::Video)?;
        let window = video_subsystem
            .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build().map_err(DirectMediaError::Canvas)?;
        let texture_creator = canvas.texture_creator();
        Ok(DirectMedia { event_pump, canvas, texture_creator, is_alive: true })
    }

    pub(crate) fn handle_events(&mut self, input: &mut Input) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.is_alive = false
                }
                Event::KeyUp { .. } | Event::KeyDown { .. } => input.handle_key_event(&event),
                _ => {}
            }
        }
        self.is_alive
    }

    pub(crate) fn present_start(&mut self) {
        self.canvas.set_draw_color(Color::RGB(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2));
        self.canvas.clear();
    }

    pub(crate) fn present_end(&mut self) {
        self.canvas.present();
        sleep(Duration::new(0, FRAME_DELAY));
    }
}

const BG_COLOR: (u8, u8, u8) = (37, 19, 26);
const FRAME_DELAY: u32 = 1_000_000_000 / 60; // 1_000 msecs / 60

#[derive(Error, Debug)]
pub enum DirectMediaError {
    #[error("context error: {0}")]
    Context(String),

    #[error("event pump error: {0}")]
    EventPump(String),

    #[error("video error: {0}")]
    Video(String),

    #[error("window error: {0}")]
    Window(#[from] WindowBuildError),

    #[error("canvas error: {0}")]
    Canvas(#[from] IntegerOrSdlError),
}
