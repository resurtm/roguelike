pub struct PlayerSpriteTextures {
    pub idle_texture_id: i32,
    pub idle_attack_texture_id: i32,
    pub walk_texture_id: i32,
    pub walk_attack_texture_id: i32,
}

impl PlayerSpriteTextures {
    pub fn new() -> PlayerSpriteTextures {
        PlayerSpriteTextures {
            idle_texture_id: -1,
            idle_attack_texture_id: -1,
            walk_texture_id: -1,
            walk_attack_texture_id: -1,
        }
    }
}
