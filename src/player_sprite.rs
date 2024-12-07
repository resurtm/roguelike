use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::direction::Direction;

pub struct PlayerSprite<'a> {
    pub position: (f64, f64),
    pub walk: bool,
    pub attack: bool,
    pub direction: Direction,
    frame: f64,

    idle_texture: Texture<'a>,
    walk_texture: Texture<'a>,
    walk_attack_texture: Texture<'a>,
    attack_texture: Texture<'a>,
}

impl PlayerSprite<'_> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> PlayerSprite {
        PlayerSprite {
            position: (0.0, 0.0),
            walk: false,
            attack: false,
            direction: Direction::Down,
            frame: 0.0,

            idle_texture: texture_creator
                .load_texture("./assets/orc/png/Orc3/orc3_idle/orc3_idle_full.png")
                .unwrap(),
            walk_texture: texture_creator
                .load_texture("./assets/orc/png/Orc3/orc3_walk/orc3_walk_full.png")
                .unwrap(),
            walk_attack_texture: texture_creator
                .load_texture("./assets/orc/png/Orc3/orc3_run_attack/orc3_run_attack_full.png")
                .unwrap(),
            attack_texture: texture_creator
                .load_texture("./assets/orc/png/Orc3/orc3_attack/orc3_attack_full.png")
                .unwrap(),
        }
    }

    fn pick_texture(&self) -> &Texture {
        if self.walk {
            return if self.attack {
                &self.walk_attack_texture
            } else {
                &self.walk_texture
            };
        }
        if self.attack {
            &self.attack_texture
        } else {
            &self.idle_texture
        }
    }

    fn pick_texture_row(&self) -> i32 {
        64 * match self.direction {
            Direction::Up => 1,
            Direction::Down => 0,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas
            .copy(
                self.pick_texture(),
                Rect::new(64 * (self.frame as i32), self.pick_texture_row(), 64, 64),
                Rect::new(self.position.0 as i32, self.position.1 as i32, 256, 256),
            )
            .unwrap();
    }

    pub fn advance(&mut self) {
        self.frame += 0.15;
        if self.frame >= 4.0 {
            self.frame = 0.0;
        }
    }
}
