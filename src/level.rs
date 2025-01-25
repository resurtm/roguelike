use crate::{aabb::Aabb, consts::TILE_SIZE};
use cgmath::Point2;
use std::collections::HashSet;
use thiserror::Error;

type Blocks = Vec<Vec<Block>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Block {
    Free,
    Wall,
    Void,
}

pub struct Level {
    blocks: Blocks,
    collision: Collision,
}

impl Level {
    /// Create a new level instance.
    pub fn new() -> Result<Self, LevelError> {
        let blocks = Self::read_blocks("./assets/level0.txt")?;
        let collision = Collision::new(&blocks);
        Ok(Self { blocks, collision })
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
}

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
