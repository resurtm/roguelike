use crate::{
    camera::Camera,
    level::{Level, LevelBlockType},
    textures::{TextureID, Textures},
};
use sdl2::{rect::Rect, render::Canvas, video::Window};
use thiserror::Error;

pub(crate) struct LevelDisplay {
    pub(crate) map: Vec<Vec<LevelDisplayCell>>,
}

impl LevelDisplay {
    pub(crate) fn new() -> LevelDisplay {
        LevelDisplay { map: Vec::new() }
    }

    pub(crate) fn sync(&mut self, level: &Level) {
        self.map = level
            .map
            .iter()
            .map(|x| {
                x.iter()
                    .map(|x| match *x {
                        LevelBlockType::Free => LevelDisplayCell::Free,
                        LevelBlockType::Wall => LevelDisplayCell::Wall,
                    })
                    .collect()
            })
            .collect()
    }

    pub(crate) fn render(
        &self,
        camera: &Camera,
        canvas: &mut Canvas<Window>,
        textures: &Textures,
    ) -> Result<(), LevelDisplayError> {
        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                let tex_id = &LOOKUP
                    .iter()
                    .find(|&t| t.0 == self.map[x][y])
                    .ok_or(LevelDisplayError::TextureLookup())?
                    .1;
                let tex = textures
                    .get(tex_id)
                    .ok_or(LevelDisplayError::TextureGet())?;

                let src = Rect::new(0, 0, 32, 32);
                let dst = Rect::new(
                    -camera.pos.x as i32 + x as i32 * TILE_SIZE as i32,
                    -camera.pos.y as i32 + y as i32 * TILE_SIZE as i32,
                    TILE_SIZE as u32,
                    TILE_SIZE as u32,
                );

                canvas
                    .copy(tex, src, dst)
                    .map_err(|msg| LevelDisplayError::CanvasCopy(msg))?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq)]
pub(crate) enum LevelDisplayCell {
    Free,
    Wall,
}

const TILE_SIZE: u8 = 32;

const LOOKUP: [(LevelDisplayCell, TextureID); 2] = [
    (LevelDisplayCell::Wall, TextureID::TileBlue),
    (LevelDisplayCell::Free, TextureID::TileRed),
];

#[derive(Error, Debug)]
pub enum LevelDisplayError {
    #[error("texture lookup error")]
    TextureLookup(),

    #[error("texture get error")]
    TextureGet(),

    #[error("canvas copy error: {0}")]
    CanvasCopy(String),
}
