use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

use crate::{direct_media::DirectMedia, types::Direction};

pub struct PlayerSprite {
    pub position: (f64, f64),
    pub direction: Direction,

    pub walk: bool,
    pub attack: bool,

    animation_frame: f64,
    idle_texture: Texture,
    walk_texture: Texture,
    walk_attack_texture: Texture,
    attack_texture: Texture,
}

impl PlayerSprite {
    pub fn new(direct_media: &mut DirectMedia) -> PlayerSprite {
        let idle_texture = direct_media
            .texture_creator
            .load_texture("./assets/orc/png/Orc3/orc3_idle/orc3_idle_full.png")
            .unwrap();
        let walk_texture = direct_media
            .texture_creator
            .load_texture("./assets/orc/png/Orc3/orc3_walk/orc3_walk_full.png")
            .unwrap();
        let walk_attack_texture = direct_media
            .texture_creator
            .load_texture("./assets/orc/png/Orc3/orc3_walk_attack/orc3_walk_attack_full.png")
            .unwrap();
        let attack_texture = direct_media
            .texture_creator
            .load_texture("./assets/orc/png/Orc3/orc3_attack/orc3_attack_full.png")
            .unwrap();

        PlayerSprite {
            position: (0.0, 0.0),
            direction: Direction::Down,

            walk: false,
            attack: false,

            animation_frame: 0.0,
            idle_texture,
            walk_texture,
            walk_attack_texture,
            attack_texture,
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
        let texture = if self.walk {
            if self.attack {
                &self.walk_attack_texture
            } else {
                &self.walk_texture
            }
        } else {
            if self.attack {
                &self.attack_texture
            } else {
                &self.idle_texture
            }
        };
        canvas
            .copy(
                texture,
                Rect::new(
                    64 * (self.animation_frame as i32),
                    self.pick_texture_row(),
                    64,
                    64,
                ),
                Rect::new(self.position.0 as i32, self.position.1 as i32, 256, 256),
            )
            .unwrap();
    }

    pub fn advance(&mut self) {
        self.animation_frame += 0.15;
        if self.animation_frame >= 4.0 {
            self.animation_frame = 0.0;
        }
    }
}
