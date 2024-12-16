use crate::types::LevelBlock;
use std::fs;

pub(crate) struct Level {
    pub(crate) map: Vec<Vec<LevelBlock>>,
}

impl Level {
    pub(crate) fn new() -> Level {
        let level = fs::read_to_string("./assets/level0.txt").expect("file");
        let lines: Vec<&str> = level.lines().collect();

        let mut map = Vec::new();
        map.resize(lines[0].len(), vec![]);
        map.iter_mut().for_each(|x| x.resize(lines.len(), LevelBlock::Void));

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                map[x][y] = match ch {
                    '.' => LevelBlock::Free,
                    '#' => LevelBlock::Wall,
                    ' ' => LevelBlock::Void,
                    _ => LevelBlock::Void,
                };
            }
        }

        Level { map }
    }
}
