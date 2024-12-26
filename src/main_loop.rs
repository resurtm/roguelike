use crate::{
    camera::Camera,
    direct_media::{DirectMedia, DirectMediaError},
    input::Input,
    level::Level,
    level_collision::{LevelCollision},
    level_display::{LevelDisplay, LevelDisplayError},
    player::Player,
    player_sprite::{PlayerSprite, PlayerSpriteError},
    textures::{Textures, TexturesError},
};
use thiserror::Error;

pub struct MainLoop<'a> {
    direct_media: DirectMedia,
    textures: Textures,
    input: Input,
    camera: Camera,

    player: Player,
    player_sprite: PlayerSprite,

    level: Level,
    level_collision: LevelCollision,
    level_display: LevelDisplay<'a>,
}

impl<'b> MainLoop<'b> {
    pub fn new<'a>() -> Result<MainLoop<'a>, MainLoopError> {
        let mut direct_media = DirectMedia::new()?;
        let textures = Textures::new(&mut direct_media)?;
        let input = Input::new();
        let camera = Camera::new();

        let player = Player::new();
        let player_sprite = PlayerSprite::new();

        let level = Level::new();
        let level_collision = LevelCollision::new(&level.map);
        let level_display = LevelDisplay::new();

        Ok(MainLoop {
            direct_media,
            textures,
            input,
            camera,

            player,
            player_sprite,

            level,
            level_collision,
            level_display,
        })
    }

    pub fn run<'a: 'b>(&'a mut self) -> Result<(), MainLoopError> {
        self.level_display.sync_level(&self.level);
        self.level_display.load_textures(&self.textures)?;

        while self.direct_media.handle_events(&mut self.input) {
            self.camera.sync_input(&self.input);
            self.camera.follow(&self.player);

            self.player.sync_input(&self.input);
            self.player_sprite.sync(&self.player);

            self.direct_media.present_start();
            self.level_display.render(&self.camera, &mut self.direct_media.canvas)?;
            self.player_sprite.render(
                &self.camera,
                &mut self.direct_media.canvas,
                &self.textures,
            )?;
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

    #[error("level display error: {0}")]
    LevelDisplayError(#[from] LevelDisplayError),

    #[error("player sprite render error: {0}")]
    PlayerSpriteRender(#[from] PlayerSpriteError),
}
