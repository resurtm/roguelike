use crate::{aabb::Aabb, consts::TILE_SIZE, types::LevelBlock as LB};
use cgmath::Point2;
use std::collections::HashSet;

pub(crate) struct LevelCollision {
    aabbs: Vec<Aabb>,
}

impl LevelCollision {
    pub(crate) fn new(m: &[Vec<LB>]) -> LevelCollision {
        let aabbs = Self::convert_wall_blocks_to_aabbs(&Self::find_wall_blocks(m));
        LevelCollision { aabbs }
    }

    fn convert_wall_blocks_to_aabbs(wall_blocks: &[(i32, i32, i32, i32)]) -> Vec<Aabb> {
        wall_blocks
            .iter()
            .map(|(x0, y0, x1, y1)| {
                Aabb::new(
                    Point2::new(*x0 as f64 * TILE_SIZE as f64, *y0 as f64 * TILE_SIZE as f64),
                    Point2::new(*x1 as f64 * TILE_SIZE as f64, *y1 as f64 * TILE_SIZE as f64),
                )
            })
            .collect()
    }

    fn find_wall_blocks(m: &[Vec<LB>]) -> Vec<(i32, i32, i32, i32)> {
        let w = m.len() as i32;
        let h = m[0].len() as i32;
        let mut v = HashSet::new();
        let mut r = vec![];
        for x in 0..w {
            for y in 0..h {
                if !v.contains(&(x, y)) && m[x as usize][y as usize] == LB::Wall {
                    r.push(Self::find_wall_block(m, &mut v, x, y));
                }
            }
        }
        r
    }

    fn find_wall_block(
        m: &[Vec<LB>],
        v: &mut HashSet<(i32, i32)>,
        sx: i32,
        sy: i32,
    ) -> (i32, i32, i32, i32) {
        let w = m.len() as i32;
        let h = m[0].len() as i32;
        let mut dim = (sx, sy, sx + 1, sy + 1);
        while (dim.0 - 1) >= 0 && Self::is_wall_block(m, v, dim.0 - 1, dim.1, dim.2, dim.3) {
            dim.0 -= 1;
        }
        while (dim.2 + 1) <= w && Self::is_wall_block(m, v, dim.0, dim.1, dim.2 + 1, dim.3) {
            dim.2 += 1;
        }
        while (dim.1 - 1) >= 0 && Self::is_wall_block(m, v, dim.0, dim.1 - 1, dim.2, dim.3) {
            dim.1 -= 1;
        }
        while (dim.3 + 1) <= h && Self::is_wall_block(m, v, dim.0, dim.1, dim.2, dim.3 + 1) {
            dim.3 += 1;
        }
        for x in dim.0..dim.2 {
            for y in dim.1..dim.3 {
                v.insert((x, y));
            }
        }
        dim
    }

    fn is_wall_block(
        m: &[Vec<LB>],
        v: &mut HashSet<(i32, i32)>,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
    ) -> bool {
        for x in x0..x1 {
            for y in y0..y1 {
                if v.contains(&(x, y)) || m[x as usize][y as usize] != LB::Wall {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::LevelCollision;
    use crate::types::LevelBlock as LB;

    #[test]
    fn test_case1() {
        let map = vec![
            vec![LB::Wall, LB::Free, LB::Free, LB::Free],
            vec![LB::Free, LB::Wall, LB::Wall, LB::Wall],
            vec![LB::Free, LB::Wall, LB::Wall, LB::Free],
            vec![LB::Free, LB::Wall, LB::Free, LB::Free],
        ];
        let actual = LevelCollision::find_wall_blocks(&map);
        let expected = vec![(0, 0, 1, 1), (1, 1, 4, 2), (1, 2, 3, 3), (1, 3, 2, 4)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_case2() {
        let map = vec![
            vec![LB::Free, LB::Wall, LB::Free, LB::Free],
            vec![LB::Wall, LB::Wall, LB::Wall, LB::Wall],
            vec![LB::Wall, LB::Wall, LB::Wall, LB::Wall],
            vec![LB::Free, LB::Free, LB::Free, LB::Wall],
        ];
        let actual = LevelCollision::find_wall_blocks(&map);
        let expected = vec![(0, 1, 3, 2), (1, 0, 3, 1), (1, 2, 3, 4), (3, 3, 4, 4)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_case3() {
        let map = vec![
            vec![LB::Wall, LB::Wall, LB::Wall, LB::Wall],
            vec![LB::Wall, LB::Wall, LB::Wall, LB::Wall],
            vec![LB::Wall, LB::Wall, LB::Wall, LB::Wall],
            vec![LB::Wall, LB::Wall, LB::Wall, LB::Wall],
        ];
        let actual = LevelCollision::find_wall_blocks(&map);
        let expected = vec![(0, 0, 4, 4)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_case4() {
        let map = vec![
            vec![LB::Wall, LB::Wall, LB::Wall, LB::Wall],
            vec![LB::Wall, LB::Free, LB::Free, LB::Wall],
            vec![LB::Wall, LB::Free, LB::Free, LB::Wall],
            vec![LB::Wall, LB::Wall, LB::Wall, LB::Wall],
        ];
        let actual = LevelCollision::find_wall_blocks(&map);
        let expected = vec![(0, 0, 4, 1), (0, 1, 1, 4), (1, 3, 4, 4), (3, 1, 4, 3)];
        assert_eq!(actual, expected);
    }
}
