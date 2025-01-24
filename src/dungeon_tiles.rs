use crate::types::LevelBlock as LB;
use cgmath::Point2;

#[derive(Clone, PartialEq)]
pub enum DungeonTile {
    TopLeftCorner,
    TopRightCorner,
    BottomRightCorner,
    BottomLeftCorner,

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

    TopLeftCornerOuter,
    TopRightCornerOuter,

    // TODO: Use these tiles below and remove #[allow(dead_code)].
    #[allow(dead_code)]
    WallTopOuter, // generic tile
    #[allow(dead_code)]
    WallTopOuter0,
    #[allow(dead_code)]
    WallTopOuter1,

    Flat, // generic tile

    Flat0,
    Flat1,
    Flat2,
    Flat3,
    Flat4,
    Flat5,
    Flat6,
    Flat7,
    Flat8,
    Flat9,
    Flat10,
    Flat11,

    Flat0Wall,
    Flat1Wall,
    Flat2Wall,
    Flat3Wall,
    Flat4Wall,
    // Flat5Wall, // not applicable
    // Flat6Wall, // not applicable
    Flat7Wall,
    Flat8Wall,
    Flat9Wall,
    Flat10Wall,
    Flat11Wall,

    Void,
}

impl DungeonTile {
    pub fn get_tex_pos(dt: &DungeonTile) -> Point2<u32> {
        match dt {
            DungeonTile::TopLeftCorner => Point2::new(0, 0),
            DungeonTile::TopRightCorner => Point2::new(5, 0),
            DungeonTile::BottomRightCorner => Point2::new(5, 4),
            DungeonTile::BottomLeftCorner => Point2::new(0, 4),

            DungeonTile::WallTop => Point2::new(1, 0), // generic tile
            DungeonTile::WallTop0 => Point2::new(1, 0),
            DungeonTile::WallTop1 => Point2::new(2, 0),
            DungeonTile::WallTop2 => Point2::new(3, 0),
            DungeonTile::WallTop3 => Point2::new(4, 0),

            DungeonTile::WallBottom => Point2::new(1, 4), // generic tile
            DungeonTile::WallBottom0 => Point2::new(1, 4),
            DungeonTile::WallBottom1 => Point2::new(2, 4),
            DungeonTile::WallBottom2 => Point2::new(3, 4),
            DungeonTile::WallBottom3 => Point2::new(4, 4),

            DungeonTile::WallLeft => Point2::new(0, 1), // generic tile
            DungeonTile::WallLeft0 => Point2::new(0, 1),
            DungeonTile::WallLeft1 => Point2::new(0, 2),
            DungeonTile::WallLeft2 => Point2::new(0, 3),

            DungeonTile::WallRight => Point2::new(5, 1), // generic tile
            DungeonTile::WallRight0 => Point2::new(5, 1),
            DungeonTile::WallRight1 => Point2::new(5, 2),
            DungeonTile::WallRight2 => Point2::new(5, 3),

            DungeonTile::TopLeftCornerOuter => Point2::new(0, 5),
            DungeonTile::TopRightCornerOuter => Point2::new(3, 5),
            DungeonTile::WallTopOuter => Point2::new(0, 5), // generic tile
            DungeonTile::WallTopOuter0 => Point2::new(1, 5),
            DungeonTile::WallTopOuter1 => Point2::new(2, 5),

            DungeonTile::Flat => Point2::new(6, 0), // generic tile

            DungeonTile::Flat0 => Point2::new(6, 0),
            DungeonTile::Flat1 => Point2::new(7, 0),
            DungeonTile::Flat2 => Point2::new(8, 0),
            DungeonTile::Flat3 => Point2::new(9, 0),
            DungeonTile::Flat4 => Point2::new(6, 1),
            DungeonTile::Flat5 => Point2::new(7, 1),
            DungeonTile::Flat6 => Point2::new(8, 1),
            DungeonTile::Flat7 => Point2::new(9, 1),
            DungeonTile::Flat8 => Point2::new(6, 2),
            DungeonTile::Flat9 => Point2::new(7, 2),
            DungeonTile::Flat10 => Point2::new(8, 2),
            DungeonTile::Flat11 => Point2::new(9, 2),

            DungeonTile::Flat0Wall => Point2::new(1, 1),
            DungeonTile::Flat1Wall => Point2::new(2, 1),
            DungeonTile::Flat2Wall => Point2::new(3, 1),
            DungeonTile::Flat3Wall => Point2::new(4, 1),
            DungeonTile::Flat4Wall => Point2::new(1, 2),
            // Flat5Wall is not applicable
            // Flat6Wall is not applicable
            DungeonTile::Flat7Wall => Point2::new(4, 2),
            DungeonTile::Flat8Wall => Point2::new(1, 3),
            DungeonTile::Flat9Wall => Point2::new(2, 3),
            DungeonTile::Flat10Wall => Point2::new(3, 3),
            DungeonTile::Flat11Wall => Point2::new(4, 3),

            DungeonTile::Void => Point2::new(8, 7),
        }
    }

    pub fn map_level_blocks_to_tiles(blocks: &[Vec<LB>]) -> Vec<Vec<DungeonTile>> {
        let w = blocks.len();
        let h = blocks[0].len();

        let mut tiles = Vec::new();
        tiles.resize(w, Vec::new());
        tiles.iter_mut().for_each(|x| x.resize(h, DungeonTile::Void));

        (0..w).for_each(|x| {
            (0..h).for_each(|y| {
                tiles[x][y] = Self::pass0(blocks, Point2::new(x, y));
            });
        });
        (0..w).for_each(|x| {
            (0..h).for_each(|y| {
                tiles[x][y] = Self::pass1(&tiles, Point2::new(x, y));
            });
        });
        (0..w).for_each(|x| {
            (0..h).for_each(|y| {
                tiles[x][y] = Self::pass2(&tiles, Point2::new(x, y));
            });
        });

        tiles
    }

    fn pass0(m: &[Vec<LB>], p: Point2<usize>) -> DungeonTile {
        let (x, t, b, l, r, tl, tr, bl, br) = Self::dirs(m, p, &LB::Void);
        if x == LB::Free {
            return DungeonTile::Flat;
        }
        // walls
        if r != LB::Free && l != LB::Free && b == LB::Free {
            return DungeonTile::WallTop;
        }
        if r != LB::Free && l != LB::Free && t == LB::Free {
            return DungeonTile::WallBottom;
        }
        if t != LB::Free && b != LB::Free && r == LB::Free {
            return DungeonTile::WallLeft;
        }
        if t != LB::Free && b != LB::Free && l == LB::Free {
            return DungeonTile::WallRight;
        }
        // corners -- inner
        if r != LB::Free && b != LB::Free && br == LB::Free {
            return DungeonTile::TopLeftCorner;
        }
        if l != LB::Free && b != LB::Free && bl == LB::Free {
            return DungeonTile::TopRightCorner;
        }
        if r != LB::Free && t != LB::Free && tr == LB::Free {
            return DungeonTile::BottomLeftCorner;
        }
        if l != LB::Free && t != LB::Free && tl == LB::Free {
            return DungeonTile::BottomRightCorner;
        }
        // corners -- outer
        if t == LB::Free && b != LB::Free && l == LB::Free && r != LB::Free {
            return DungeonTile::TopLeftCornerOuter;
        }
        if t == LB::Free && b != LB::Free && l != LB::Free && r == LB::Free {
            return DungeonTile::TopRightCornerOuter;
        }
        if t != LB::Free && b == LB::Free && l == LB::Free && r != LB::Free
            || t != LB::Free && b == LB::Free && l != LB::Free && r == LB::Free
        {
            return DungeonTile::WallTop;
        }
        DungeonTile::Void
    }

    fn pass1(m: &[Vec<DungeonTile>], p: Point2<usize>) -> DungeonTile {
        let (x, _, _, _, _, _, _, _, _) = Self::dirs(m, p, &DungeonTile::Void);
        match x {
            DungeonTile::Flat => FLAT_TILE_LOOKUP[p.x % 4 + p.y % 3 * 3].clone(),
            DungeonTile::WallTop => WALL_TOP_TILE_LOOKUP[p.x % 4].clone(),
            DungeonTile::WallBottom => WALL_BOTTOM_TILE_LOOKUP[p.x % 4].clone(),
            DungeonTile::WallLeft => WALL_LEFT_TILE_LOOKUP[p.y % 3].clone(),
            DungeonTile::WallRight => WALL_RIGHT_TILE_LOOKUP[p.y % 3].clone(),
            _ => x,
        }
    }

    fn pass2(m: &[Vec<DungeonTile>], p: Point2<usize>) -> DungeonTile {
        let (x, t, b, l, r, _, _, _, _) = Self::dirs(m, p, &DungeonTile::Void);
        if Self::flat(&x) {
            // corners
            if !Self::flat(&t) && Self::flat(&b) && !Self::flat(&l) && Self::flat(&r) {
                return DungeonTile::Flat0Wall;
            }
            if !Self::flat(&t) && Self::flat(&b) && Self::flat(&l) && !Self::flat(&r) {
                return DungeonTile::Flat3Wall;
            }
            if Self::flat(&t) && !Self::flat(&b) && !Self::flat(&l) && Self::flat(&r) {
                return DungeonTile::Flat8Wall;
            }
            if Self::flat(&t) && !Self::flat(&b) && Self::flat(&l) && !Self::flat(&r) {
                return DungeonTile::Flat11Wall;
            }

            // walls
            if Self::flat(&t) && Self::flat(&b) && !Self::flat(&l) && Self::flat(&r) {
                return DungeonTile::Flat4Wall;
            }
            if Self::flat(&t) && Self::flat(&b) && Self::flat(&l) && !Self::flat(&r) {
                return DungeonTile::Flat7Wall;
            }
            if Self::flat(&t) && !Self::flat(&b) && Self::flat(&l) && Self::flat(&r) {
                return DungeonTile::Flat9Wall;
            }
            if !Self::flat(&t) && Self::flat(&l) && Self::flat(&r) {
                return DungeonTile::Flat1Wall;
            }
        }
        x
    }

    fn dirs<T: Clone>(m: &[Vec<T>], p: Point2<usize>, d: &T) -> (T, T, T, T, T, T, T, T, T) {
        let (w, h) = (m.len() - 1, m[0].len() - 1);
        let x = &m[p.x][p.y];

        let t = if p.y == 0 { d } else { &m[p.x][p.y - 1] };
        let b = if p.y == h { d } else { &m[p.x][p.y + 1] };
        let l = if p.x == 0 { d } else { &m[p.x - 1][p.y] };
        let r = if p.x == w { d } else { &m[p.x + 1][p.y] };

        let tl = if p.x == 0 || p.y == 0 { d } else { &m[p.x - 1][p.y - 1] };
        let tr = if p.x == w || p.y == 0 { d } else { &m[p.x + 1][p.y - 1] };
        let bl = if p.x == 0 || p.y == h { d } else { &m[p.x - 1][p.y + 1] };
        let br = if p.x == w || p.y == h { d } else { &m[p.x + 1][p.y + 1] };

        (
            x.clone(),
            t.clone(),
            b.clone(),
            l.clone(),
            r.clone(),
            tl.clone(),
            tr.clone(),
            bl.clone(),
            br.clone(),
        )
    }

    fn flat(t: &DungeonTile) -> bool {
        FLAT_TILE_LOOKUP.contains(t) || FLAT_WALL_TILE_LOOKUP.contains(t)
    }
}

const FLAT_TILE_LOOKUP: [DungeonTile; 12] = [
    // row 0
    DungeonTile::Flat0, // 0x0
    DungeonTile::Flat1, // 1x0
    DungeonTile::Flat2, // 2x0
    DungeonTile::Flat3, // 3x0
    // row 1
    DungeonTile::Flat4, // 0x1
    DungeonTile::Flat5, // 1x1
    DungeonTile::Flat6, // 2x1
    DungeonTile::Flat7, // 3x1
    // row 2
    DungeonTile::Flat8,  // 0x2
    DungeonTile::Flat9,  // 1x2
    DungeonTile::Flat10, // 2x2
    DungeonTile::Flat11, // 3x2
];
const FLAT_WALL_TILE_LOOKUP: [DungeonTile; 12] = [
    // row 0
    DungeonTile::Flat0Wall, // 0x0
    DungeonTile::Flat1Wall, // 1x0
    DungeonTile::Flat2Wall, // 2x0
    DungeonTile::Flat3Wall, // 3x0
    // row 1
    DungeonTile::Flat4Wall, // 0x1
    DungeonTile::Flat5,     // 1x1
    DungeonTile::Flat6,     // 2x1
    DungeonTile::Flat7Wall, // 3x1
    // row 2
    DungeonTile::Flat8Wall,  // 0x2
    DungeonTile::Flat9Wall,  // 1x2
    DungeonTile::Flat10Wall, // 2x2
    DungeonTile::Flat11Wall, // 3x2
];
const WALL_TOP_TILE_LOOKUP: [DungeonTile; 4] = [
    DungeonTile::WallTop0, // 1x0
    DungeonTile::WallTop1, // 2x0
    DungeonTile::WallTop2, // 3x0
    DungeonTile::WallTop3, // 4x0
];
const WALL_BOTTOM_TILE_LOOKUP: [DungeonTile; 4] = [
    DungeonTile::WallBottom0, // 1x4
    DungeonTile::WallBottom1, // 2x4
    DungeonTile::WallBottom2, // 3x4
    DungeonTile::WallBottom3, // 4x4
];
const WALL_LEFT_TILE_LOOKUP: [DungeonTile; 3] = [
    DungeonTile::WallLeft0, // 0x1
    DungeonTile::WallLeft1, // 0x2
    DungeonTile::WallLeft2, // 0x3
];
const WALL_RIGHT_TILE_LOOKUP: [DungeonTile; 3] = [
    DungeonTile::WallRight0, // 5x1
    DungeonTile::WallRight1, // 5x2
    DungeonTile::WallRight2, // 5x3
];
