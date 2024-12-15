use crate::{
    camera::Camera,
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

    pub(crate) fn sync(&mut self, level: &Level) {
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
        camera: &Camera,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), LevelDisplayError> {
        let tex =
            self.tex_dungeon.ok_or(LevelDisplayError::TextureGet(TextureID::DungeonTileset))?;
        for x in 0..self.tiles.len() {
            for y in 0..self.tiles[x].len() {
                let pos = DungeonTile::get_pos(&self.tiles[x][y]);
                let src = Rect::new(pos.x * 16, pos.y * 16, 16, 16);
                let dst = Rect::new(
                    -camera.position.x as i32 + 1920 / 2 + x as i32 * TILE_SIZE as i32,
                    -camera.position.y as i32 + 1200 / 2 + y as i32 * TILE_SIZE as i32,
                    TILE_SIZE as u32,
                    TILE_SIZE as u32,
                );
                canvas.copy(tex, src, dst).map_err(LevelDisplayError::CanvasCopy)?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum LevelDisplayCell {
    WallTopLeft,
    WallTopRight,
    WallBottomLeft,
    WallBottomRight,

    WallTopLeftOuter,
    WallTopRightOuter,

    WallTop, // generic tile
    WallTop0,
    WallTop1,
    WallTop2,
    WallTop3,

    WallBottom, // generic tile
    WallBottom0,
    WallBottom1,
    WallBottom2,
    WallBottom3,

    WallLeft, // generic tile
    WallLeft0,
    WallLeft1,
    WallLeft2,

    WallRight, // generic tile
    WallRight0,
    WallRight1,
    WallRight2,

    Floor, // generic tile
    Floor00,
    Floor10,
    Floor20,
    Floor30,
    Floor01,
    Floor11,
    Floor21,
    Floor31,
    Floor02,
    Floor12,
    Floor22,
    Floor32,

    NotAvailable,
}

const LOOKUP: [(LevelDisplayCell, u8, u8); 33] = [
    (LevelDisplayCell::WallTopLeft, 0, 0),
    (LevelDisplayCell::WallTopRight, 5, 0),
    (LevelDisplayCell::WallBottomLeft, 0, 4),
    (LevelDisplayCell::WallBottomRight, 5, 4),
    (LevelDisplayCell::WallTopLeftOuter, 0, 5),
    (LevelDisplayCell::WallTopRightOuter, 3, 5),
    (LevelDisplayCell::WallTop0, 1, 0),
    (LevelDisplayCell::WallTop1, 2, 0),
    (LevelDisplayCell::WallTop2, 3, 0),
    (LevelDisplayCell::WallTop3, 4, 0),
    (LevelDisplayCell::WallBottom0, 1, 4),
    (LevelDisplayCell::WallBottom1, 2, 4),
    (LevelDisplayCell::WallBottom2, 3, 4),
    (LevelDisplayCell::WallBottom3, 4, 4),
    (LevelDisplayCell::WallLeft0, 0, 1),
    (LevelDisplayCell::WallLeft1, 0, 2),
    (LevelDisplayCell::WallLeft2, 0, 3),
    (LevelDisplayCell::WallRight0, 5, 1),
    (LevelDisplayCell::WallRight1, 5, 2),
    (LevelDisplayCell::WallRight2, 5, 3),
    (LevelDisplayCell::Floor, 1, 1),
    (LevelDisplayCell::Floor00, 6, 0),
    (LevelDisplayCell::Floor10, 7, 0),
    (LevelDisplayCell::Floor20, 8, 0),
    (LevelDisplayCell::Floor30, 9, 0),
    (LevelDisplayCell::Floor01, 6, 1),
    (LevelDisplayCell::Floor11, 7, 1),
    (LevelDisplayCell::Floor21, 8, 1),
    (LevelDisplayCell::Floor31, 9, 1),
    (LevelDisplayCell::Floor02, 6, 2),
    (LevelDisplayCell::Floor12, 7, 2),
    (LevelDisplayCell::Floor22, 8, 2),
    (LevelDisplayCell::Floor32, 9, 2),
];

const FLOOR_LOOKUP: [[LevelDisplayCell; 4]; 3] = [
    [
        LevelDisplayCell::Floor00,
        LevelDisplayCell::Floor10,
        LevelDisplayCell::Floor20,
        LevelDisplayCell::Floor30,
    ],
    [
        LevelDisplayCell::Floor01,
        LevelDisplayCell::Floor11,
        LevelDisplayCell::Floor21,
        LevelDisplayCell::Floor31,
    ],
    [
        LevelDisplayCell::Floor02,
        LevelDisplayCell::Floor12,
        LevelDisplayCell::Floor22,
        LevelDisplayCell::Floor32,
    ],
];

const TILE_SIZE: u8 = 96;

#[derive(Error, Debug)]
pub enum LevelDisplayError {
    #[error("texture lookup error")]
    TextureLookup(),

    #[error("texture get error: {0}")]
    TextureGet(TextureID),

    #[error("canvas copy error: {0}")]
    CanvasCopy(String),
}
