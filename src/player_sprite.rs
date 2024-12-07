use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::direction::Direction;

pub struct PlayerSprite<'a> {
    pub position: (f64, f64),
    direction: Direction,
    frame: f64,
    idle_texture: Texture<'a>,
}

impl PlayerSprite<'_> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> PlayerSprite {
        PlayerSprite {
            position: (0.0, 0.0),
            direction: Direction::Down,
            frame: 0.0,
            idle_texture: texture_creator
                .load_texture("./assets/orc/png/Orc3/orc3_idle/orc3_idle_full.png")
                .unwrap(),
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas
            .copy(
                &self.idle_texture,
                Rect::new(64 * (self.frame as i32), 0, 64, 64),
                Rect::new(self.position.0 as i32, self.position.1 as i32, 128, 128),
            )
            .unwrap();
    }

    pub fn advance(&mut self) {
        self.frame += 0.05;
        if self.frame >= 4.0 {
            self.frame = 0.0;
        }
    }
}
