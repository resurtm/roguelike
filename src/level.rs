use crate::video::{TextureGroup, Vertex, Video};
use crate::{aabb::Aabb, consts::TILE_SIZE};
use cgmath::Point2;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashSet;
use thiserror::Error;
use wgpu::util::DeviceExt;

// --------------------------------------------------
// --- BLOCKS ---
// --------------------------------------------------

type Blocks = Vec<Vec<Block>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Block {
    Free,
    Wall,
    Void,
}

// --------------------------------------------------
// --- LEVEL ---
// --------------------------------------------------

pub struct Level {
    blocks: Blocks,
    collision: Collision,
    pub mesh: Mesh,
}

impl Level {
    /// Create a new level instance.
    pub fn new(video: &Video) -> Result<Self, LevelError> {
        let blocks = Self::read_blocks("./assets/level0.txt")?;
        let collision = Collision::new(&blocks);
        let mesh = Mesh::new(video, &DungeonTile::map_blocks_to_dungeon_tiles(&blocks))?;
        Ok(Self { blocks, collision, mesh })
    }

    /// Load level blocks from a file.
    fn read_blocks(file_path: &str) -> Result<Blocks, std::io::Error> {
        let lines = std::fs::read_to_string(file_path)?;
        let lines: Vec<_> = lines.lines().collect();
        let mut blocks: Blocks = vec![vec![Block::Void; lines.len()]; lines[0].len()];
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.bytes().enumerate() {
                blocks[x][y] = match ch {
                    b'.' => Block::Free,
                    b'#' => Block::Wall,
                    b' ' | _ => Block::Void,
                };
            }
        }
        Ok(blocks)
    }
}

#[derive(Error, Debug)]
pub enum LevelError {
    #[error("read blocks error: {0}")]
    ReadBlocks(#[from] std::io::Error),

    #[error("mesh error: {0}")]
    Mesh(#[from] MeshError),
}

// --------------------------------------------------
// --- COLLISION ---
// --------------------------------------------------

pub struct Collision {
    aabbs: Vec<Aabb>,
}

impl Collision {
    /// Create a new level collision.
    pub fn new(blocks: &Blocks) -> Self {
        let aabbs = Self::build_aabbs_from_block_sets(&Self::find_block_sets(blocks));
        Self { aabbs }
    }

    /// Take a slice of block sets and convert them into AABBs vector.
    /// A block set is a 4 elements tuple of signed 32 bit integers.
    /// First two elements are x and y of min point,
    /// and second two elements are x and y of max point.
    fn build_aabbs_from_block_sets(blocks: &[(i32, i32, i32, i32)]) -> Vec<Aabb> {
        blocks
            .iter()
            .map(|(x0, y0, x1, y1)| {
                Aabb::new(
                    Point2::new(*x0 as f32 * TILE_SIZE as f32, *y0 as f32 * TILE_SIZE as f32),
                    Point2::new(*x1 as f32 * TILE_SIZE as f32, *y1 as f32 * TILE_SIZE as f32),
                )
            })
            .collect()
    }

    /// Takes raw blocks and builds a vector of block sets.
    /// This is done by some kind of breadth first search algorithm.
    /// A block set is a 4 elements tuple of signed 32 bit integers.
    /// First two elements are x and y of min point,
    /// and second two elements are x and y of max point.
    fn find_block_sets(blocks: &Blocks) -> Vec<(i32, i32, i32, i32)> {
        let (w, h) = (blocks.len() as i32, blocks[0].len() as i32);
        let (mut visited, mut result) = (HashSet::new(), vec![]);
        for x in 0..w {
            for y in 0..h {
                if !visited.contains(&(x, y)) && blocks[x as usize][y as usize] == Block::Wall {
                    result.push(Self::find_block_sets_internal(blocks, w, h, &mut visited, x, y));
                }
            }
        }
        result
    }

    /// Internal helper for the [`find_block_sets`].
    fn find_block_sets_internal(
        blocks: &Blocks,
        w: i32,
        h: i32,
        visited: &mut HashSet<(i32, i32)>,
        x: i32,
        y: i32,
    ) -> (i32, i32, i32, i32) {
        let mut bs = (x, y, x + 1, y + 1);
        while (bs.0 - 1) >= 0 && Self::is_block_set(blocks, visited, bs.0 - 1, bs.1, bs.2, bs.3) {
            bs.0 -= 1;
        }
        while (bs.2 + 1) <= w && Self::is_block_set(blocks, visited, bs.0, bs.1, bs.2 + 1, bs.3) {
            bs.2 += 1;
        }
        while (bs.1 - 1) >= 0 && Self::is_block_set(blocks, visited, bs.0, bs.1 - 1, bs.2, bs.3) {
            bs.1 -= 1;
        }
        while (bs.3 + 1) <= h && Self::is_block_set(blocks, visited, bs.0, bs.1, bs.2, bs.3 + 1) {
            bs.3 += 1;
        }
        for i in bs.0..bs.2 {
            for j in bs.1..bs.3 {
                visited.insert((i, j));
            }
        }
        bs
    }

    /// Internal helper for the [`find_block_sets`] and [`find_block_sets_internal`].
    fn is_block_set(
        blocks: &Blocks,
        visited: &mut HashSet<(i32, i32)>,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
    ) -> bool {
        for x in x0..x1 {
            for y in y0..y1 {
                if visited.contains(&(x, y)) || blocks[x as usize][y as usize] != Block::Wall {
                    return false;
                }
            }
        }
        true
    }
}

// --------------------------------------------------
// --- DUNGEON TILE ---
// --------------------------------------------------

type DungeonTiles = Vec<Vec<DungeonTile>>;

#[derive(Clone, Copy, PartialEq, Debug)]
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

    // TODO: Use this dungeon tile below and remove #[allow(dead_code)].
    #[allow(dead_code)]
    WallTopOuter, // generic tile
    // TODO: Use this dungeon tile below and remove #[allow(dead_code)].
    #[allow(dead_code)]
    WallTopOuter0,
    // TODO: Use this dungeon tile below and remove #[allow(dead_code)].
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
    // Flat5Wall, // not applicable & not available
    // Flat6Wall, // not applicable & not available
    Flat7Wall,
    Flat8Wall,
    Flat9Wall,
    Flat10Wall,
    Flat11Wall,

    Void,
}

impl DungeonTile {
    /// Takes a dungeon tile and returns a texture position.
    /// A texture position is in 0..=11 (x/u)+ 0..=1 (y/v) space.
    pub fn get_texture_position(dt: &Self) -> Point2<u32> {
        match dt {
            Self::TopLeftCorner => Point2::new(0, 0),
            Self::TopRightCorner => Point2::new(5, 0),
            Self::BottomRightCorner => Point2::new(5, 4),
            Self::BottomLeftCorner => Point2::new(0, 4),

            Self::WallTop => Point2::new(1, 0), // generic tile
            Self::WallTop0 => Point2::new(1, 0),
            Self::WallTop1 => Point2::new(2, 0),
            Self::WallTop2 => Point2::new(3, 0),
            Self::WallTop3 => Point2::new(4, 0),

            Self::WallBottom => Point2::new(1, 4), // generic tile
            Self::WallBottom0 => Point2::new(1, 4),
            Self::WallBottom1 => Point2::new(2, 4),
            Self::WallBottom2 => Point2::new(3, 4),
            Self::WallBottom3 => Point2::new(4, 4),

            Self::WallLeft => Point2::new(0, 1), // generic tile
            Self::WallLeft0 => Point2::new(0, 1),
            Self::WallLeft1 => Point2::new(0, 2),
            Self::WallLeft2 => Point2::new(0, 3),

            Self::WallRight => Point2::new(5, 1), // generic tile
            Self::WallRight0 => Point2::new(5, 1),
            Self::WallRight1 => Point2::new(5, 2),
            Self::WallRight2 => Point2::new(5, 3),

            Self::TopLeftCornerOuter => Point2::new(0, 5),
            Self::TopRightCornerOuter => Point2::new(3, 5),
            Self::WallTopOuter => Point2::new(0, 5), // generic tile
            Self::WallTopOuter0 => Point2::new(1, 5),
            Self::WallTopOuter1 => Point2::new(2, 5),

            Self::Flat => Point2::new(6, 0), // generic tile

            Self::Flat0 => Point2::new(6, 0),
            Self::Flat1 => Point2::new(7, 0),
            Self::Flat2 => Point2::new(8, 0),
            Self::Flat3 => Point2::new(9, 0),
            Self::Flat4 => Point2::new(6, 1),
            Self::Flat5 => Point2::new(7, 1),
            Self::Flat6 => Point2::new(8, 1),
            Self::Flat7 => Point2::new(9, 1),
            Self::Flat8 => Point2::new(6, 2),
            Self::Flat9 => Point2::new(7, 2),
            Self::Flat10 => Point2::new(8, 2),
            Self::Flat11 => Point2::new(9, 2),

            Self::Flat0Wall => Point2::new(1, 1),
            Self::Flat1Wall => Point2::new(2, 1),
            Self::Flat2Wall => Point2::new(3, 1),
            Self::Flat3Wall => Point2::new(4, 1),
            Self::Flat4Wall => Point2::new(1, 2),
            // Self::Flat5Wall is not applicable & not available
            // Self::Flat6Wall is not applicable & not available
            Self::Flat7Wall => Point2::new(4, 2),
            Self::Flat8Wall => Point2::new(1, 3),
            Self::Flat9Wall => Point2::new(2, 3),
            Self::Flat10Wall => Point2::new(3, 3),
            Self::Flat11Wall => Point2::new(4, 3),

            Self::Void => Point2::new(8, 7),
        }
    }

    /// Maps blocks to dungeon tiles.
    pub fn map_blocks_to_dungeon_tiles(blocks: &Blocks) -> DungeonTiles {
        let (w, h) = (blocks.len(), blocks[0].len());
        let mut result: DungeonTiles = vec![vec![DungeonTile::Void; h]; w];
        for p in 0..3 {
            for x in 0..w {
                for y in 0..h {
                    result[x][y] = match p {
                        0 => Self::pass0(blocks, (x, y).into()),
                        1 => Self::pass1(&result, (x, y).into()),
                        2 => Self::pass2(&result, (x, y).into()),
                        _ => panic!("incorrect pass"),
                    }
                }
            }
        }
        result
    }

    /// Internal helper for [`map_blocks_to_dungeon_tiles`], pass 0.
    fn pass0(blocks: &Blocks, point: Point2<usize>) -> Self {
        let (x, t, b, l, r, tl, tr, bl, br) = Self::directions(blocks, point, Block::Void);
        if x == Block::Free {
            return Self::Flat;
        }

        // walls
        if r != Block::Free && l != Block::Free && b == Block::Free {
            return Self::WallTop;
        }
        if r != Block::Free && l != Block::Free && t == Block::Free {
            return Self::WallBottom;
        }
        if t != Block::Free && b != Block::Free && r == Block::Free {
            return Self::WallLeft;
        }
        if t != Block::Free && b != Block::Free && l == Block::Free {
            return Self::WallRight;
        }

        // corners -- inner
        if r != Block::Free && b != Block::Free && br == Block::Free {
            return Self::TopLeftCorner;
        }
        if l != Block::Free && b != Block::Free && bl == Block::Free {
            return Self::TopRightCorner;
        }
        if r != Block::Free && t != Block::Free && tr == Block::Free {
            return Self::BottomLeftCorner;
        }
        if l != Block::Free && t != Block::Free && tl == Block::Free {
            return Self::BottomRightCorner;
        }

        // corners -- outer
        if t == Block::Free && b != Block::Free && l == Block::Free && r != Block::Free {
            return Self::TopLeftCornerOuter;
        }
        if t == Block::Free && b != Block::Free && l != Block::Free && r == Block::Free {
            return Self::TopRightCornerOuter;
        }
        if t != Block::Free && b == Block::Free && l == Block::Free && r != Block::Free
            || t != Block::Free && b == Block::Free && l != Block::Free && r == Block::Free
        {
            return Self::WallTop;
        }

        Self::Void
    }

    /// Internal helper for [`map_blocks_to_dungeon_tiles`], pass 1.
    fn pass1(dungeon_tiles: &DungeonTiles, point: Point2<usize>) -> Self {
        let x = Self::directions(dungeon_tiles, point, Self::Void).0;
        match x {
            Self::Flat => DUNGEON_TILE_FLAT_LOOKUP[point.x % 4 + point.y % 3 * 3],
            Self::WallTop => DUNGEON_TILE_WALL_TOP_LOOKUP[point.x % 4],
            Self::WallBottom => DUNGEON_TILE_WALL_BOTTOM_LOOKUP[point.x % 4],
            Self::WallLeft => DUNGEON_TILE_WALL_LEFT_LOOKUP[point.y % 3],
            Self::WallRight => DUNGEON_TILE_WALL_RIGHT_LOOKUP[point.y % 3],
            _ => x,
        }
    }

    /// Internal helper for [`map_blocks_to_dungeon_tiles`], pass 2.
    fn pass2(dungeon_tiles: &DungeonTiles, point: Point2<usize>) -> Self {
        let (x, t, b, l, r, _, _, _, _) = Self::directions(dungeon_tiles, point, Self::Void);
        if Self::is_flat(&x) {
            // corners
            if !Self::is_flat(&t) && Self::is_flat(&b) && !Self::is_flat(&l) && Self::is_flat(&r) {
                return Self::Flat0Wall;
            }
            if !Self::is_flat(&t) && Self::is_flat(&b) && Self::is_flat(&l) && !Self::is_flat(&r) {
                return Self::Flat3Wall;
            }
            if Self::is_flat(&t) && !Self::is_flat(&b) && !Self::is_flat(&l) && Self::is_flat(&r) {
                return Self::Flat8Wall;
            }
            if Self::is_flat(&t) && !Self::is_flat(&b) && Self::is_flat(&l) && !Self::is_flat(&r) {
                return Self::Flat11Wall;
            }

            // walls
            if Self::is_flat(&t) && Self::is_flat(&b) && !Self::is_flat(&l) && Self::is_flat(&r) {
                return Self::Flat4Wall;
            }
            if Self::is_flat(&t) && Self::is_flat(&b) && Self::is_flat(&l) && !Self::is_flat(&r) {
                return Self::Flat7Wall;
            }
            if Self::is_flat(&t) && !Self::is_flat(&b) && Self::is_flat(&l) && Self::is_flat(&r) {
                return Self::Flat9Wall;
            }
            if !Self::is_flat(&t) && Self::is_flat(&l) && Self::is_flat(&r) {
                return Self::Flat1Wall;
            }
        }
        x
    }

    /// Internal helper for [`pass0`], [`pass1`], and [`pass2`].
    fn directions<T: Copy>(m: &[Vec<T>], p: Point2<usize>, def: T) -> (T, T, T, T, T, T, T, T, T) {
        let (w, h) = (m.len() - 1, m[0].len() - 1);
        let x = m[p.x][p.y];

        let t = if p.y == 0 { def } else { m[p.x][p.y - 1] };
        let b = if p.y == h { def } else { m[p.x][p.y + 1] };
        let l = if p.x == 0 { def } else { m[p.x - 1][p.y] };
        let r = if p.x == w { def } else { m[p.x + 1][p.y] };

        let tl = if p.x == 0 || p.y == 0 { def } else { m[p.x - 1][p.y - 1] };
        let tr = if p.x == w || p.y == 0 { def } else { m[p.x + 1][p.y - 1] };
        let bl = if p.x == 0 || p.y == h { def } else { m[p.x - 1][p.y + 1] };
        let br = if p.x == w || p.y == h { def } else { m[p.x + 1][p.y + 1] };

        // Center/current (x), top (t), bottom (b), left (l), right (r).
        // Top-left (tl), top-right (tr), bottom-left (bl), bottom-right (br).
        (x, t, b, l, r, tl, tr, bl, br)
    }

    /// Internal helper for [`pass2`].
    fn is_flat(t: &DungeonTile) -> bool {
        DUNGEON_TILE_FLAT_LOOKUP.contains(t) || DUNGEON_TILE_FLAT_WALL_LOOKUP.contains(t)
    }
}

const DUNGEON_TILE_FLAT_LOOKUP: [DungeonTile; 12] = [
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
const DUNGEON_TILE_FLAT_WALL_LOOKUP: [DungeonTile; 12] = [
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
const DUNGEON_TILE_WALL_TOP_LOOKUP: [DungeonTile; 4] = [
    DungeonTile::WallTop0, // 1x0
    DungeonTile::WallTop1, // 2x0
    DungeonTile::WallTop2, // 3x0
    DungeonTile::WallTop3, // 4x0
];
const DUNGEON_TILE_WALL_BOTTOM_LOOKUP: [DungeonTile; 4] = [
    DungeonTile::WallBottom0, // 1x4
    DungeonTile::WallBottom1, // 2x4
    DungeonTile::WallBottom2, // 3x4
    DungeonTile::WallBottom3, // 4x4
];
const DUNGEON_TILE_WALL_LEFT_LOOKUP: [DungeonTile; 3] = [
    DungeonTile::WallLeft0, // 0x1
    DungeonTile::WallLeft1, // 0x2
    DungeonTile::WallLeft2, // 0x3
];
const DUNGEON_TILE_WALL_RIGHT_LOOKUP: [DungeonTile; 3] = [
    DungeonTile::WallRight0, // 5x1
    DungeonTile::WallRight1, // 5x2
    DungeonTile::WallRight2, // 5x3
];

// --------------------------------------------------
// --- MESH ---
// --------------------------------------------------

pub struct Mesh {
    pub texture: TextureGroup,

    pub vertex_buffer: wgpu::Buffer,
    #[allow(dead_code)]
    vertex_count: u32,

    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,

    buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl Mesh {
    /// Create a new instance of level mesh.
    pub fn new(video: &Video, dungeon_tiles: &DungeonTiles) -> Result<Self, MeshError> {
        // texture
        let texture =
            TextureGroup::new(video, include_bytes!("../assets/dungeon/Dungeon_Tileset.png"))?;

        // geometry -- vertices
        let (vertices, vertex_count) = Self::build_vertices(dungeon_tiles);
        let vertex_buffer = video.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("level_mesh_vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        // geometry -- indices
        let (indices, index_count) = Self::build_indices(vertex_count);
        let index_buffer = video.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("level_mesh_index_buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let buffer = video.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("level_mesh_buffer"),
            size: std::mem::size_of::<[crate::video::MatrixUniform; 1]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = video.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("level_mesh_bind_group"),
            layout: &video.bind_group_layouts[2],
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
        });

        Ok(Self {
            texture,
            vertex_buffer,
            vertex_count,
            index_buffer,
            index_count,
            buffer,
            bind_group,
        })
    }

    /// Build vertices vector to be used to create a new vertex buffer.
    /// Internal helper for [`new`].
    fn build_vertices(dungeon_tiles: &DungeonTiles) -> (Vec<Vertex>, u32) {
        let (m, n) = (MESH_TEXTURE_SIZE as f32, MESH_TEXTURE_TILE_SIZE as f32);
        let (mut vertices, mut vertex_count) = (vec![], 0);
        for (x, its) in dungeon_tiles.iter().enumerate() {
            for (y, it) in its.iter().enumerate() {
                let (x, y) = (x as f32, y as f32);
                let (u, v) = DungeonTile::get_texture_position(it).into();
                let (u, v) = (u as f32, v as f32);
                vertices.push(Vertex::new(
                    (x - MESH_XZ_COORD, MESH_Y_COORD, y - MESH_XZ_COORD).into(),
                    ((n * u) / m, n * v / m).into(),
                ));
                vertices.push(Vertex::new(
                    (x - MESH_XZ_COORD, MESH_Y_COORD, y + MESH_XZ_COORD).into(),
                    ((n * u) / m, (n * (v + 1.0)) / n).into(),
                ));
                vertices.push(Vertex::new(
                    (x + MESH_XZ_COORD, MESH_Y_COORD, y + MESH_XZ_COORD).into(),
                    ((n * (u + 1.0)) / n, (n * (v + 1.0)) / n).into(),
                ));
                vertices.push(Vertex::new(
                    (x + MESH_XZ_COORD, MESH_Y_COORD, y - MESH_XZ_COORD).into(),
                    ((n * (u + 1.0)) / m, (n * v) / m).into(),
                ));
                vertex_count += MESH_VERTICES_PER_TILE;
            }
        }
        (vertices, vertex_count)
    }

    /// Build indices vector to be used to create a new index buffer.
    /// Internal helper for [`new`].
    fn build_indices(vertex_count: u32) -> (Vec<u16>, u32) {
        let (mut indices, mut index_count) = (vec![], 0);
        for i in 0..(vertex_count / MESH_VERTICES_PER_TILE) as u16 {
            let t = [0, 2, 3, 0, 1, 2].iter().map(|x| *x + i * MESH_VERTICES_PER_TILE as u16);
            indices.extend(t);
            index_count += MESH_INDICES_PER_TILE;
        }
        (indices, index_count)
    }
}

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("texture error: {0}")]
    Texture(#[from] crate::video::TextureError),
}

const MESH_XZ_COORD: f32 = 0.5;
const MESH_Y_COORD: f32 = -0.5;
const MESH_TEXTURE_SIZE: u32 = 160;
const MESH_TEXTURE_TILE_SIZE: u32 = 16;
const MESH_VERTICES_PER_TILE: u32 = 4;
const MESH_INDICES_PER_TILE: u32 = 6;

// --------------------------------------------------
// --- TEST ---
// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{Block, Collision};

    #[test]
    fn test_collision_case1() {
        let blocks = vec![
            vec![Block::Wall, Block::Free, Block::Free, Block::Free],
            vec![Block::Free, Block::Wall, Block::Wall, Block::Wall],
            vec![Block::Free, Block::Wall, Block::Wall, Block::Free],
            vec![Block::Free, Block::Wall, Block::Free, Block::Free],
        ];
        let actual = Collision::find_block_sets(&blocks);
        let expected = vec![(0, 0, 1, 1), (1, 1, 4, 2), (1, 2, 3, 3), (1, 3, 2, 4)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_collision_case2() {
        let blocks = vec![
            vec![Block::Free, Block::Wall, Block::Free, Block::Free],
            vec![Block::Wall, Block::Wall, Block::Wall, Block::Wall],
            vec![Block::Wall, Block::Wall, Block::Wall, Block::Wall],
            vec![Block::Free, Block::Free, Block::Free, Block::Wall],
        ];
        let actual = Collision::find_block_sets(&blocks);
        let expected = vec![(0, 1, 3, 2), (1, 0, 3, 1), (1, 2, 3, 4), (3, 3, 4, 4)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_collision_case3() {
        let blocks = vec![
            vec![Block::Wall, Block::Wall, Block::Wall, Block::Wall],
            vec![Block::Wall, Block::Wall, Block::Wall, Block::Wall],
            vec![Block::Wall, Block::Wall, Block::Wall, Block::Wall],
            vec![Block::Wall, Block::Wall, Block::Wall, Block::Wall],
        ];
        let actual = Collision::find_block_sets(&blocks);
        let expected = vec![(0, 0, 4, 4)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_collision_case4() {
        let blocks = vec![
            vec![Block::Wall, Block::Wall, Block::Wall, Block::Wall],
            vec![Block::Wall, Block::Free, Block::Free, Block::Wall],
            vec![Block::Wall, Block::Free, Block::Free, Block::Wall],
            vec![Block::Wall, Block::Wall, Block::Wall, Block::Wall],
        ];
        let actual = Collision::find_block_sets(&blocks);
        let expected = vec![(0, 0, 4, 1), (0, 1, 1, 4), (1, 3, 4, 4), (3, 1, 4, 3)];
        assert_eq!(actual, expected);
    }
}
