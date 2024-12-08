use sdl2::{
    rect::Rect,
    render::Canvas,
    video::Window,
};

use crate::{
    player::{Player, PlayerMovementState},
    textures::{TextureID, Textures},
    types::Direction,
};

pub struct PlayerSprite {
    location: (f64, f64),
    direction: Direction,
    state: PlayerSpriteState,
    animation_frame: f64,
}

impl PlayerSprite {
    pub fn new() -> PlayerSprite {
        PlayerSprite {
            location: (0.0, 0.0),
            direction: Direction::Down,
            state: PlayerSpriteState::Idle,
            animation_frame: 0.0,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, textures: &Textures) {
        let texture = textures
            .get(
                &LOOKUP
                    .iter()
                    .find(|&x| x.0 == self.state)
                    .expect("texture 1")
                    .1,
            )
            .expect("texture 2");

        let texture_row = match self.direction {
            Direction::Up => 1,
            Direction::Down => 0,
            Direction::Left => 2,
            Direction::Right => 3,
        };

        canvas
            .copy(
                texture,
                Rect::new(64 * (self.animation_frame as i32), 64 * texture_row, 64, 64),
                Rect::new(self.location.0 as i32, self.location.1 as i32, 256, 256),
            )
            .unwrap();
    }

    pub fn advance(&mut self, player: &Player) {
        self.state = match player.movement_state {
            PlayerMovementState::Idle => {
                if player.is_attack {
                    PlayerSpriteState::IdleAttack
                } else {
                    PlayerSpriteState::Idle
                }
            }
            PlayerMovementState::Walk => {
                if player.is_attack {
                    PlayerSpriteState::WalkAttack
                } else {
                    PlayerSpriteState::Walk
                }
            }
            PlayerMovementState::Run => {
                if player.is_attack {
                    PlayerSpriteState::RunAttack
                } else {
                    PlayerSpriteState::Run
                }
            }
        };

        self.location = player.position;
        self.direction = Self::find_direction(player);

        self.animation_frame += ANIMATION_SPEED;
        if self.animation_frame >= ANIMATION_FRAMES as f64 {
            self.animation_frame = 0.0;
        }
    }

    fn find_direction(player: &Player) -> Direction {
        if player.velocity.0 < 0.0 {
            if player.velocity.0.abs() > player.velocity.1.abs() {
                return Direction::Left;
            }
            return if player.velocity.1 < 0.0 {
                return Direction::Up;
            } else {
                return Direction::Down;
            };
        }
        if player.velocity.0 > 0.0 {
            if player.velocity.0.abs() > player.velocity.1.abs() {
                return Direction::Right;
            }
            return if player.velocity.1 < 0.0 {
                return Direction::Up;
            } else {
                return Direction::Down;
            };
        }
        Direction::Down
    }
}

const ANIMATION_SPEED: f64 = 0.15;
const ANIMATION_FRAMES: i32 = 4;

#[derive(PartialEq)]
enum PlayerSpriteState {
    Idle,       // orc3_idle
    IdleAttack, // orc3_attack
    Walk,       // orc3_walk
    WalkAttack, // orc3_walk_attack
    Run,        // orc3_run
    RunAttack,  // orc3_run_attack
    Hurt,       // orc3_hurt
    Death,      // orc3_death
}

const LOOKUP: [(PlayerSpriteState, TextureID); 8] = [
    (PlayerSpriteState::Idle, TextureID::Orc3Idle),
    (PlayerSpriteState::IdleAttack, TextureID::Orc3Attack),
    (PlayerSpriteState::Walk, TextureID::Orc3Walk),
    (PlayerSpriteState::WalkAttack, TextureID::Orc3WalkAttack),
    (PlayerSpriteState::Run, TextureID::Orc3Run),
    (PlayerSpriteState::RunAttack, TextureID::Orc3RunAttack),
    (PlayerSpriteState::Hurt, TextureID::Orc3Hurt),
    (PlayerSpriteState::Death, TextureID::Orc3Death),
];
