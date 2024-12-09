use std::fs;

pub struct Level {
    pub cells: Vec<Vec<LevelCellType>>,
}

impl Level {
    pub fn new() -> Level {
        let level = fs::read_to_string("./assets/level.txt").expect("file");
        let lines: Vec<&str> = level.lines().collect();

        let mut cells = Vec::new();
        for _ in 0..lines[0].len() {
            let mut row = Vec::new();
            for _ in 0..lines.len() {
                row.push(LevelCellType::Free);
            }
            cells.push(row);
        }

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().into_iter().enumerate() {
                cells[x][y] = match ch {
                    '#' => LevelCellType::Wall,
                    '.' | _ => LevelCellType::Free,
                };
            }
        }

        Level { cells }
    }
}

#[derive(Debug)]
pub enum LevelCellType {
    Free,
    Wall,
}
