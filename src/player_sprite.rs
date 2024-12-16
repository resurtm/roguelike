use crate::{
    camera::Camera,
    consts::{WINDOW_HEIGHT, WINDOW_WIDTH},
    player::Player,
    textures::{TextureID, Textures},
    types::Direction,
};
use cgmath::{EuclideanSpace, InnerSpace, Point2};
use sdl2::{rect::Rect, render::Canvas, video::Window};
use thiserror::Error;

pub(crate) struct PlayerSprite {
    location: Point2<f32>,
    direction: Direction,
    state: PlayerSpriteState,
    animation_frame: f32,
}

impl PlayerSprite {
    pub(crate) fn new() -> PlayerSprite {
        PlayerSprite {
            location: Point2::origin(),
            direction: Direction::Down,
            state: PlayerSpriteState::Idle,
            animation_frame: 0.0,
        }
    }

    pub(crate) fn render(
        &self,
        cam: &Camera,
        can: &mut Canvas<Window>,
        tex: &Textures,
    ) -> Result<(), PlayerSpriteError> {
        let tex = tex.get(&self.find_texture_id()).ok_or(PlayerSpriteError::TextureGet())?;
        let src = Rect::new(
            SRC_SIZE as i32 * (self.animation_frame as i32),
            SRC_SIZE as i32 * self.find_texture_row() as i32,
            SRC_SIZE,
            SRC_SIZE,
        );
        let dst = Rect::new(
            (WINDOW_WIDTH / 2) as i32 - cam.position.x as i32 + self.location.x as i32
                - (DST_SIZE / 2) as i32,
            (WINDOW_HEIGHT / 2) as i32 - cam.position.y as i32 + self.location.y as i32
                - (DST_SIZE / 2) as i32,
            DST_SIZE,
            DST_SIZE,
        );

        can.copy(tex, src, dst).map_err(PlayerSpriteError::CanvasCopy)?;
        Ok(())
    }

    pub(crate) fn sync(&mut self, player: &Player) {
        self.location = player.position;
        self.direction = Self::find_direction(player);
        self.state = Self::find_state(player);

        self.animation_frame += ANIMATION_SPEED;
        if self.animation_frame >= ANIMATION_FRAMES as f32 {
            self.animation_frame = 0.0;
        }
    }

    fn find_state(player: &Player) -> PlayerSpriteState {
        let speed = player.velocity.magnitude2();
        if speed >= RUN_SPEED_THRESHOLD {
            return if player.is_attack {
                PlayerSpriteState::RunAttack
            } else {
                PlayerSpriteState::Run
            };
        } else if speed >= WALK_SPEED_THRESHOLD {
            return if player.is_attack {
                PlayerSpriteState::WalkAttack
            } else {
                PlayerSpriteState::Walk
            };
        }
        if player.is_attack {
            PlayerSpriteState::IdleAttack
        } else {
            PlayerSpriteState::Idle
        }
    }

    fn find_direction(player: &Player) -> Direction {
        if player.velocity.x < 0.0 {
            if player.velocity.x.abs() > player.velocity.y.abs() {
                return Direction::Left;
            }
            return if player.velocity.y < 0.0 { Direction::Up } else { Direction::Down };
        }
        if player.velocity.x > 0.0 {
            if player.velocity.x.abs() > player.velocity.y.abs() {
                return Direction::Right;
            }
            return if player.velocity.y < 0.0 { Direction::Up } else { Direction::Down };
        }
        Direction::Down
    }

    fn find_texture_row(&self) -> u32 {
        match self.direction {
            Direction::Up => 1,
            Direction::Down => 0,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }

    fn find_texture_id(&self) -> TextureID {
        match self.state {
            PlayerSpriteState::Idle => TextureID::Orc3Idle,
            PlayerSpriteState::IdleAttack => TextureID::Orc3Attack,
            PlayerSpriteState::Walk => TextureID::Orc3Walk,
            PlayerSpriteState::WalkAttack => TextureID::Orc3WalkAttack,
            PlayerSpriteState::Run => TextureID::Orc3Run,
            PlayerSpriteState::RunAttack => TextureID::Orc3RunAttack,
            PlayerSpriteState::Hurt => TextureID::Orc3Hurt,
            PlayerSpriteState::Death => TextureID::Orc3Death,
        }
    }
}

const WALK_SPEED_THRESHOLD: f32 = 0.5;
const RUN_SPEED_THRESHOLD: f32 = 2.5;

const ANIMATION_SPEED: f32 = 0.15;
const ANIMATION_FRAMES: u32 = 4;

const SRC_SIZE: u32 = 64;
const DST_SIZE: u32 = 192;

#[derive(PartialEq)]
enum PlayerSpriteState {
    Idle,       // orc3_idle
    IdleAttack, // orc3_attack
    Walk,       // orc3_walk
    WalkAttack, // orc3_walk_attack
    Run,        // orc3_run
    RunAttack,  // orc3_run_attack
    // TODO: Use states below.
    Hurt,  // orc3_hurt
    Death, // orc3_death
}

#[derive(Error, Debug)]
pub enum PlayerSpriteError {
    #[error("texture get error")]
    TextureGet(),

    #[error("canvas copy error: {0}")]
    CanvasCopy(String),
}
