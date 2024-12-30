use crate::{
    camera::Camera,
    consts::{TILE_SIZE, TILE_TEX_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH},
    dungeon_tiles::DungeonTile,
    level::Level,
    level_collision::LevelCollision,
    textures::{TextureID, Textures},
};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};
use thiserror::Error;

pub(crate) struct LevelDisplay<'a> {
    pub(crate) tiles: Vec<Vec<DungeonTile>>,

    tex_dungeon: Option<&'a Texture>,
}

impl<'a> LevelDisplay<'a> {
    pub(crate) fn new<'b>() -> LevelDisplay<'b> {
        LevelDisplay { tiles: Vec::new(), tex_dungeon: None }
    }

    pub(crate) fn prepare(
        &mut self,
        level: &Level,
        textures: &'a Textures,
    ) -> Result<(), LevelDisplayError> {
        self.tiles = DungeonTile::map_level_blocks_to_tiles(&level.map);
        self.tex_dungeon = Some(
            textures
                .get(&TextureID::DungeonTileset)
                .ok_or(LevelDisplayError::TextureGet(TextureID::DungeonTileset))?,
        );
        Ok(())
    }

    pub(crate) fn render_tiles(
        &self,
        cam: &Camera,
        can: &mut Canvas<Window>,
    ) -> Result<(), LevelDisplayError> {
        let tex =
            self.tex_dungeon.ok_or(LevelDisplayError::TextureGet(TextureID::DungeonTileset))?;
        for x in 0..self.tiles.len() {
            for y in 0..self.tiles[x].len() {
                let pos = DungeonTile::get_tex_pos(&self.tiles[x][y]);
                let src = Rect::new(
                    (pos.x * TILE_TEX_SIZE) as i32,
                    (pos.y * TILE_TEX_SIZE) as i32,
                    TILE_TEX_SIZE,
                    TILE_TEX_SIZE,
                );
                let dst = Rect::new(
                    (WINDOW_WIDTH / 2) as i32 - cam.position.x as i32 + x as i32 * TILE_SIZE as i32,
                    (WINDOW_HEIGHT / 2) as i32 - cam.position.y as i32
                        + y as i32 * TILE_SIZE as i32,
                    TILE_SIZE,
                    TILE_SIZE,
                );
                can.copy(tex, src, dst).map_err(LevelDisplayError::CanvasCopy)?;
            }
        }
        Ok(())
    }

    pub(crate) fn render_collision_debug(
        &self,
        cam: &Camera,
        can: &mut Canvas<Window>,
        col: &LevelCollision,
    ) -> Result<(), LevelDisplayError> {
        can.set_draw_color(Color::RGB(255, 0, 0));
        for aabb in col.aabbs.iter() {
            can.draw_rect(Rect::new(
                (WINDOW_WIDTH / 2) as i32 - cam.position.x as i32 + aabb.min.x as i32,
                (WINDOW_HEIGHT / 2) as i32 - cam.position.y as i32 + aabb.min.y as i32,
                (aabb.max.x - aabb.min.x) as u32,
                (aabb.max.y - aabb.min.y) as u32,
            ))
            .map_err(LevelDisplayError::CanvasCopy)?;
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum LevelDisplayError {
    #[error("texture get error: {0}")]
    TextureGet(TextureID),

    #[error("canvas copy error: {0}")]
    CanvasCopy(String),
}
