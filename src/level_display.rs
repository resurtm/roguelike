use crate::{
    camera::Camera,
    consts::{WINDOW_HEIGHT, WINDOW_WIDTH},
    dungeon_tiles::DungeonTile,
    level::Level,
    textures::{TextureID, Textures},
};
use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};
use thiserror::Error;

pub(crate) struct LevelDisplay<'a> {
    pub(crate) tiles: Vec<Vec<DungeonTile>>,

    tex_dungeon: Option<&'a Texture>,
}

impl<'b> LevelDisplay<'b> {
    pub(crate) fn new<'a>() -> LevelDisplay<'a> {
        LevelDisplay { tiles: Vec::new(), tex_dungeon: None }
    }

    pub(crate) fn sync_level(&mut self, level: &Level) {
        self.tiles = DungeonTile::map_level_blocks(&level.map);
    }

    pub(crate) fn load_textures<'a: 'b>(
        &mut self,
        textures: &'a Textures,
    ) -> Result<(), LevelDisplayError> {
        self.tex_dungeon = Some(
            textures
                .get(&TextureID::DungeonTileset)
                .ok_or(LevelDisplayError::TextureGet(TextureID::DungeonTileset))?,
        );
        Ok(())
    }

    pub(crate) fn render(
        &self,
        cam: &Camera,
        can: &mut Canvas<Window>,
    ) -> Result<(), LevelDisplayError> {
        let tex =
            self.tex_dungeon.ok_or(LevelDisplayError::TextureGet(TextureID::DungeonTileset))?;
        for x in 0..self.tiles.len() {
            for y in 0..self.tiles[x].len() {
                let pos = DungeonTile::get_pos(&self.tiles[x][y]);
                let src = Rect::new(
                    (pos.x * SRC_SIZE) as i32,
                    (pos.y * SRC_SIZE) as i32,
                    SRC_SIZE,
                    SRC_SIZE,
                );
                let dst = Rect::new(
                    (WINDOW_WIDTH / 2) as i32 - cam.position.x as i32 + x as i32 * DST_SIZE as i32,
                    (WINDOW_HEIGHT / 2) as i32 - cam.position.y as i32 + y as i32 * DST_SIZE as i32,
                    DST_SIZE,
                    DST_SIZE,
                );
                can.copy(tex, src, dst).map_err(LevelDisplayError::CanvasCopy)?;
            }
        }
        Ok(())
    }
}

const SRC_SIZE: u32 = 16;
const DST_SIZE: u32 = 96;

#[derive(Error, Debug)]
pub enum LevelDisplayError {
    #[error("texture get error: {0}")]
    TextureGet(TextureID),

    #[error("canvas copy error: {0}")]
    CanvasCopy(String),
}
