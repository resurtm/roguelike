use std::fs;

pub(crate) struct Level {
    pub(crate) map: Vec<Vec<LevelBlockType>>,
}

impl Level {
    pub(crate) fn new() -> Level {
        let level = fs::read_to_string("./assets/level.txt").expect("file");
        let lines: Vec<&str> = level.lines().collect();

        let mut map = Vec::new();
        for _ in 0..lines[0].len() {
            let mut row = Vec::new();
            for _ in 0..lines.len() {
                row.push(LevelBlockType::Free);
            }
            map.push(row);
        }

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().into_iter().enumerate() {
                map[x][y] = match ch {
                    '#' => LevelBlockType::Wall,
                    '.' => LevelBlockType::Free,
                    ' ' | _ => LevelBlockType::Void,
                };
            }
        }

        Level { map }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum LevelBlockType {
    Wall,
    Free,
    Void,
}
