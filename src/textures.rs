use sdl2::{image::LoadTexture, render::Texture};
use std::{collections::HashMap, fmt};
use thiserror::Error;

use crate::direct_media::DirectMedia;

pub(crate) struct Textures {
    map: HashMap<TextureID, Texture>,
}

impl Textures {
    pub(crate) fn new(direct_media: &mut DirectMedia) -> Result<Textures, TexturesError> {
        let mut map = HashMap::new();

        for (texture_id, path) in LOOKUP.iter() {
            let texture = direct_media
                .texture_creator
                .load_texture(path)
                .map_err(|err_msg| TexturesError::Load(format!("{:?}", texture_id), err_msg))?;
            map.insert(texture_id.clone(), texture);
        }

        Ok(Textures { map })
    }

    pub(crate) fn get(&self, texture_id: &TextureID) -> Option<&Texture> {
        self.map.get(texture_id)
    }
}

const LOOKUP: [(TextureID, &str); 13] = [
    (TextureID::Orc3Attack, "./assets/orc/png/Orc3/orc3_attack/orc3_attack_full.png"),
    (TextureID::Orc3Death, "./assets/orc/png/Orc3/orc3_death/orc3_death_full.png"),
    (TextureID::Orc3Hurt, "./assets/orc/png/Orc3/orc3_hurt/orc3_hurt_full.png"),
    (TextureID::Orc3Idle, "./assets/orc/png/Orc3/orc3_idle/orc3_idle_full.png"),
    (TextureID::Orc3Run, "./assets/orc/png/Orc3/orc3_run/orc3_run_full.png"),
    (TextureID::Orc3RunAttack, "./assets/orc/png/Orc3/orc3_run_attack/orc3_run_attack_full.png"),
    (TextureID::Orc3Walk, "./assets/orc/png/Orc3/orc3_walk/orc3_walk_full.png"),
    (TextureID::Orc3WalkAttack, "./assets/orc/png/Orc3/orc3_walk_attack/orc3_walk_attack_full.png"),
    (TextureID::TileRed, "./assets/tile/tile-red.png"),
    (TextureID::TileGreen, "./assets/tile/tile-green.png"),
    (TextureID::TileBlue, "./assets/tile/tile-blue.png"),
    (TextureID::TileNotAvailable, "./assets/tile/tile-na.png"),
    (TextureID::DungeonTileset, "./assets/dungeon/Dungeon_Tileset.png"),
];

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) enum TextureID {
    Orc3Attack,
    Orc3Death,
    Orc3Hurt,
    Orc3Idle,
    Orc3Run,
    Orc3RunAttack,
    Orc3Walk,
    Orc3WalkAttack,

    TileRed,
    TileGreen,
    TileBlue,
    TileNotAvailable,

    DungeonTileset,
}

impl fmt::Display for TextureID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Error, Debug)]
pub enum TexturesError {
    #[error("load error: {0} {1}")]
    Load(String, String),
}
