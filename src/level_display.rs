use crate::{
    camera::Camera,
    level::{Level, LevelBlockType as BT},
    textures::{TextureID, Textures},
};
use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};
use thiserror::Error;

pub(crate) struct LevelDisplay<'a> {
    pub(crate) map: Vec<Vec<LevelDisplayCell>>,

    tex_dung: Option<&'a Texture>,
    tex_na: Option<&'a Texture>,
}

impl<'b> LevelDisplay<'b> {
    pub(crate) fn new<'a>() -> LevelDisplay<'a> {
        LevelDisplay {
            map: Vec::new(),
            tex_dung: None,
            tex_na: None,
        }
    }

    pub(crate) fn sync(&mut self, level: &Level) {
        self.map.clear();
        self.map.resize(level.map.len(), Vec::new());
        self.map
            .iter_mut()
            .for_each(|x| x.resize(level.map[0].len(), LevelDisplayCell::NotAvailable));

        for (x, its) in level.map.iter().enumerate() {
            for (y, it) in its.iter().enumerate() {
                let cell = Self::pick_display_cell(&level.map, it.clone(), x as i32, y as i32);
                self.map[x][y] = match cell {
                    LevelDisplayCell::Floor => FLOOR_LOOKUP[y % 3][x % 4].clone(),
                    _ => cell,
                }
            }
        }
    }

    fn pick_display_cell(m: &Vec<Vec<BT>>, c: BT, x: i32, y: i32) -> LevelDisplayCell {
        let w = m.len() as i32;
        let h = m[0].len() as i32;

        let p: Vec<BT> = vec![
            (x, y - 1),     // top
            (x + 1, y),     // right
            (x, y + 1),     // bottom
            (x - 1, y),     // left
            (x + 1, y - 1), // top right
            (x + 1, y + 1), // bottom right
            (x - 1, y + 1), // bottom left
            (x - 1, y - 1), // top left
        ]
        .iter()
        .map(|(i, j)| {
            if *i < 0 || *j < 0 || *i == w || *j == h {
                BT::Void
            } else {
                m[*i as usize][*j as usize].clone()
            }
        })
        .collect();

        const T: usize = 0;
        const R: usize = 1;
        const B: usize = 2;
        const L: usize = 3;
        const TR: usize = 4;
        const BR: usize = 5;
        const BL: usize = 6;
        const TL: usize = 7;

        if c == BT::Wall {
            // straight vertical and horizontal
            if p[R] == BT::Wall && p[L] == BT::Wall && p[B] == BT::Free {
                return LevelDisplayCell::WallTop0;
            }
            if p[T] == BT::Wall && p[B] == BT::Wall && p[L] == BT::Free {
                return LevelDisplayCell::WallRight0;
            }
            if p[R] == BT::Wall && p[L] == BT::Wall && p[T] == BT::Free {
                return LevelDisplayCell::WallBottom0;
            }
            if p[T] == BT::Wall && p[B] == BT::Wall && p[R] == BT::Free {
                return LevelDisplayCell::WallLeft0;
            }

            // outward corners
            if p[T] == BT::Free && p[L] == BT::Free && p[R] == BT::Wall && p[B] == BT::Wall {
                return LevelDisplayCell::WallTopLeftOuter;
            }
            if p[T] == BT::Free && p[L] == BT::Wall && p[R] == BT::Free && p[B] == BT::Wall {
                return LevelDisplayCell::WallTopRightOuter;
            }
            if p[T] == BT::Wall && p[L] == BT::Free && p[R] == BT::Wall && p[B] == BT::Free
                || p[T] == BT::Wall && p[L] == BT::Wall && p[R] == BT::Free && p[B] == BT::Free
            {
                return LevelDisplayCell::WallTop0;
            }

            // inward corners
            if p[B] == BT::Wall && p[L] == BT::Wall && p[BL] == BT::Free {
                return LevelDisplayCell::WallTopRight;
            }
            if p[T] == BT::Wall && p[L] == BT::Wall && p[TL] == BT::Free {
                return LevelDisplayCell::WallBottomRight;
            }
            if p[T] == BT::Wall && p[R] == BT::Wall && p[TR] == BT::Free {
                return LevelDisplayCell::WallBottomLeft;
            }
            if p[B] == BT::Wall && p[R] == BT::Wall && p[BR] == BT::Free {
                return LevelDisplayCell::WallTopLeft;
            }
        }

        if c == BT::Free {
            return LevelDisplayCell::Floor;
        }

        LevelDisplayCell::NotAvailable
    }

    pub(crate) fn load_textures<'a: 'b>(
        &mut self,
        textures: &'a Textures,
    ) -> Result<(), LevelDisplayError> {
        self.tex_dung = Some(
            textures
                .get(&TextureID::DungeonTileset)
                .ok_or(LevelDisplayError::TextureGet(TextureID::DungeonTileset))?,
        );
        self.tex_na = Some(
            textures
                .get(&TextureID::TileNotAvailable)
                .ok_or(LevelDisplayError::TextureGet(TextureID::TileNotAvailable))?,
        );
        Ok(())
    }

    pub(crate) fn render(
        &self,
        camera: &Camera,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), LevelDisplayError> {
        let tex_dung = self
            .tex_dung
            .ok_or(LevelDisplayError::TextureGet(TextureID::DungeonTileset))?;
        let tex_na = self
            .tex_na
            .ok_or(LevelDisplayError::TextureGet(TextureID::TileNotAvailable))?;

        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                let cell = &self.map[x][y];
                let tex = if *cell == LevelDisplayCell::NotAvailable {
                    tex_na
                } else {
                    tex_dung
                };
                let src = if *cell == LevelDisplayCell::NotAvailable {
                    Rect::new(0, 0, 32, 32)
                } else {
                    let pos = &LOOKUP
                        .iter()
                        .find(|&x| x.0 == *cell)
                        .ok_or(LevelDisplayError::TextureLookup())?;
                    Rect::new(pos.1 as i32 * 16, pos.2 as i32 * 16, 16, 16)
                };
                let dst = Rect::new(
                    -camera.position.x as i32 + (1920 / 2) as i32 + x as i32 * TILE_SIZE as i32,
                    -camera.position.y as i32 + (1200 / 2) as i32 + y as i32 * TILE_SIZE as i32,
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
