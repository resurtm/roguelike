use thiserror::Error;

pub struct Scene {
    pub observer: crate::observer::Observer,
    pub level: crate::level::Level,
    pub player: crate::player::Player,
}

impl Scene {
    pub fn new(video: &crate::video::Video) -> Result<Self, SceneError> {
        let observer = crate::observer::Observer::new(video);
        let level = crate::level::Level::new(video)?;
        let player = crate::player::Player::new(video)?;
        Ok(Self { observer, level, player })
    }

    pub fn advance(&mut self, video: &crate::video::Video, input: &crate::input::Input) {
        self.observer.update(video);
        self.player.advance();
        self.player.apply_input(input);
    }
}

#[derive(Error, Debug)]
pub enum SceneError {
    #[error("level error: {0}")]
    Level(#[from] crate::level::LevelError),

    #[error("player error: {0}")]
    Player(#[from] crate::player::PlayerError),
}
