use crate::{
    level::{Level, LevelError},
    player::{Player, PlayerError},
    video::{ObserverGroup, Video},
};
use thiserror::Error;

pub struct Scene {
    pub observer: ObserverGroup,
    pub level: Level,
    pub player: Player,
}

impl Scene {
    pub fn new(video: &Video) -> Result<Self, SceneError> {
        let observer = ObserverGroup::new(video);
        let level = Level::new(video)?;
        let player = Player::new(video)?;
        Ok(Self { observer, level, player })
    }

    pub fn update(&mut self) {
        self.player.advance();
    }
}

#[derive(Error, Debug)]
pub enum SceneError {
    #[error("level error: {0}")]
    Level(#[from] LevelError),

    #[error("player error: {0}")]
    Player(#[from] PlayerError),
}
