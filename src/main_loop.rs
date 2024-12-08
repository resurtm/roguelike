use crate::direct_media::{DirectMedia, DirectMediaError};
use crate::textures::{Textures, TexturesError};
use crate::{input::Input, player::Player, player_sprite::PlayerSprite};
use thiserror::Error;

pub struct MainLoop {
    direct_media: DirectMedia,
    textures: Textures,
    input: Input,

    player: Player,
    player_sprite: PlayerSprite,
}

impl MainLoop {
    pub fn new() -> Result<MainLoop, MainLoopError> {
        let mut direct_media = DirectMedia::new()?;
        let textures = Textures::new(&mut direct_media)?;
        let input = Input::new();

        let player = Player::new();
        let player_sprite = PlayerSprite::new();

        Ok(MainLoop {
            direct_media,
            textures,
            input,

            player,
            player_sprite,
        })
    }

    pub fn run(&mut self) {
        while self.direct_media.handle_events(&mut self.input) {
            self.player.advance(&self.input);
            self.player_sprite.advance(&self.player);

            self.direct_media.present_start();
            self.player_sprite
                .render(&mut self.direct_media.canvas, &self.textures);
            self.direct_media.present_end();
        }
    }
}

#[derive(Error, Debug)]
pub enum MainLoopError {
    #[error("direct media error: {0}")]
    DirectMedia(#[from] DirectMediaError),

    #[error("textures error: {0}")]
    Textures(#[from] TexturesError),
}
