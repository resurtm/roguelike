use crate::{
    level::{Level, LevelError},
    video::{ObserverGroup, Video},
};
use thiserror::Error;

pub struct Scene {
    pub observer: ObserverGroup,
    pub level: Level,
}

impl Scene {
    pub fn new(video: &Video) -> Result<Self, SceneError> {
        let observer = ObserverGroup::new(video);
        let level = Level::new(video)?;
        Ok(Self { observer, level })
    }
}

#[derive(Error, Debug)]
pub enum SceneError {
    #[error("level error: {0}")]
    Level(#[from] LevelError),
}
