use crate::{
    direct_media::{DirectMedia, DirectMediaError},
    level::Level,
    level_display::{LevelDisplay, LevelDisplayError},
    player_sprite::PlayerSpriteError,
    textures::{Textures, TexturesError},
    {input::Input, player::Player, player_sprite::PlayerSprite},
};
use thiserror::Error;

pub struct MainLoop {
    direct_media: DirectMedia,
    textures: Textures,
    input: Input,

    player: Player,
    player_sprite: PlayerSprite,
    level: Level,
    level_draw: LevelDisplay,
}

impl MainLoop {
    pub fn new() -> Result<MainLoop, MainLoopError> {
        let mut direct_media = DirectMedia::new()?;
        let textures = Textures::new(&mut direct_media)?;
        let input = Input::new();

        let player = Player::new();
        let player_sprite = PlayerSprite::new();
        let level = Level::new();
        let level_draw = LevelDisplay::new();

        Ok(MainLoop {
            direct_media,
            textures,
            input,

            player,
            player_sprite,
            level,
            level_draw,
        })
    }

    pub fn run(&mut self) -> Result<(), MainLoopError> {
        while self.direct_media.handle_events(&mut self.input) {
            self.player.advance(&self.input);
            self.player_sprite.advance(&self.player);
            self.level_draw.sync(&self.level);

            self.direct_media.present_start();
            self.level_draw
                .render(&mut self.direct_media.canvas, &self.textures)?;
            self.player_sprite
                .render(&mut self.direct_media.canvas, &self.textures)?;
            self.direct_media.present_end();
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum MainLoopError {
    #[error("direct media error: {0}")]
    DirectMedia(#[from] DirectMediaError),

    #[error("textures error: {0}")]
    Textures(#[from] TexturesError),

    #[error("player sprite render error: {0}")]
    PlayerSpriteRender(#[from] PlayerSpriteError),

    #[error("level sprite render error: {0}")]
    LevelDisplayError(#[from] LevelDisplayError),
}
