pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum LevelBlock {
    Free,
    Wall,
    Void,
}
