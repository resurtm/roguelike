use crate::direct_media::{DirectMedia, DirectMediaError};
use crate::{input::Input, player::Player, player_sprite::PlayerSprite, types::Direction};
use thiserror::Error;

pub struct MainLoop {
    input: Input,
    direct_media: DirectMedia,

    player: Player,
    player_sprite: PlayerSprite,
}

impl MainLoop {
    pub fn new() -> Result<MainLoop, MainLoopError> {
        let input = Input::new();
        let mut direct_media = DirectMedia::new()?;

        let player = Player::new();
        let player_sprite = PlayerSprite::new(&mut direct_media);

        Ok(MainLoop {
            input,
            direct_media,
            player,
            player_sprite,
        })
    }

    pub fn run(&mut self) {
        while self.direct_media.handle_events(&mut self.input) {
            if self.input.key_up {
                self.player.thrust(Direction::Up);
            }
            if self.input.key_down {
                self.player.thrust(Direction::Down);
            }
            if self.input.key_left {
                self.player.thrust(Direction::Left);
            }
            if self.input.key_right {
                self.player.thrust(Direction::Right);
            }

            self.player.advance();
            self.player_sprite.advance();

            self.player_sprite.position = self.player.position;
            self.player_sprite.walk = self.player.is_walk();
            self.player_sprite.attack = self.input.key_space;
            self.player_sprite.direction = self.player.get_effective_direction();

            self.direct_media.present_start();
            self.player_sprite.render(&mut self.direct_media.canvas);
            self.direct_media.present_end();
        }
    }
}

#[derive(Error, Debug)]
pub enum MainLoopError {
    #[error("direct media error: {0}")]
    DirectMedia(#[from] DirectMediaError),
}
