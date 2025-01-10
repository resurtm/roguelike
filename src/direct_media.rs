use crate::{
    consts::{WINDOW_SIZE, WINDOW_TITLE},
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
use thiserror::Error;

pub(crate) struct DirectMedia {
    events: EventPump,
    pub(crate) canvas: Canvas<Window>,
    pub(crate) tex_creator: TextureCreator<WindowContext>,
    is_alive: bool,
}

impl DirectMedia {
    pub(crate) fn new() -> Result<Self, DirectMediaError> {
        let context = sdl2::init().map_err(DirectMediaError::Context)?;
        let events = context.event_pump().map_err(DirectMediaError::EventPump)?;
        let video = context.video().map_err(DirectMediaError::Video)?;
        let window =
            video.window(WINDOW_TITLE, WINDOW_SIZE.0, WINDOW_SIZE.1).position_centered().build()?;
        let canvas = window.into_canvas().build().map_err(DirectMediaError::Canvas)?;
        let tex_creator = canvas.texture_creator();
        Ok(Self { events, canvas, tex_creator, is_alive: true })
    }

    pub(crate) fn handle_events(&mut self, input: &mut Input) -> bool {
        for event in self.events.poll_iter() {
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
        std::thread::sleep(std::time::Duration::new(0, FRAME_DELAY_NSECS));
    }
}

const BG_COLOR: (u8, u8, u8) = (37, 19, 26);
const FRAME_DELAY_NSECS: u32 = 1_000_000_000 / 60; // 1_000 msecs / 60

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
