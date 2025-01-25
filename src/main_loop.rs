use crate::{
    camera::Camera,
    direct_media::{DirectMedia, DirectMediaError},
    input::Input,
    level::Level,
    level_collision::LevelCollision,
    level_display::{LevelDisplay, LevelDisplayError},
    player::Player,
    player_sprite::{PlayerDisplay, PlayerSpriteError},
    textures::{Textures, TexturesError},
};
use thiserror::Error;

pub struct MainLoop<'a> {
    direct_media: DirectMedia,
    textures: Textures,
    input: Input,
    camera: Camera,

    player: Player,
    player_display: PlayerDisplay,

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
        let player_display = PlayerDisplay::new();

        let level = Level::new();
        let level_display = LevelDisplay::new();
        let level_collision = LevelCollision::new(&level.blocks);

        Ok(MainLoop {
            direct_media,
            textures,
            input,
            camera,

            player,
            player_display,

            level,
            level_collision,
            level_display,
        })
    }

    pub fn run<'a: 'b>(&'a mut self) -> Result<(), MainLoopError> {
        self.level_display.prepare(&self.level, &self.textures)?;

        while self.direct_media.handle_events(&mut self.input) {
            self.camera.apply_input(&self.input);
            self.camera.follow_player(&self.player);

            self.player.apply_input(&self.input);
            self.player.sync_level_collision(&self.level_collision);
            self.player_display.sync(&self.player);

            self.direct_media.present_start();
            self.level_display.render_tiles(&self.camera, &mut self.direct_media.canvas)?;
            self.level_display.render_collision_debug(
                &self.camera,
                &mut self.direct_media.canvas,
                &self.level_collision,
            )?;
            self.player_display.render_player(
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
