use crate::{geometry::Direction, input::Input, video};
use cgmath::{InnerSpace, Point2, Vector2};
use thiserror::Error;
use wgpu::util::DeviceExt;

// --------------------------------------------------
// --- PLAYER ---
// --------------------------------------------------

/// Represents the player character.
pub struct Player {
    pub position: Point2<f32>,

    velocity: Vector2<f32>,
    velocity_delta: f32,
    velocity_max: f32,
    velocity_slowdown: f32,

    attack: bool,

    pub mesh: Mesh,
}

impl Player {
    /// Creates a new player character instance.
    pub fn new(video: &crate::video::Video) -> Result<Self, PlayerError> {
        let mesh = Mesh::new(video)?;

        Ok(Self {
            position: SPAWN_POSITION.into(),

            velocity: Vector2::new(0.0, 0.0),
            velocity_delta: 0.01,
            velocity_max: 0.025,
            velocity_slowdown: 0.92,

            attack: false,

            mesh,
        })
    }

    /// Advance the player character state, physics, etc.
    pub fn advance(&mut self) {
        self.position += self.velocity;
        self.velocity *= self.velocity_slowdown;

        if self.velocity.x > self.velocity_max {
            self.velocity.x = self.velocity_max;
        }
        if self.velocity.x < -self.velocity_max {
            self.velocity.x = -self.velocity_max;
        }
        if self.velocity.y > self.velocity_max {
            self.velocity.y = self.velocity_max;
        }
        if self.velocity.y < -self.velocity_max {
            self.velocity.y = -self.velocity_max;
        }

        self.mesh.advance(self.position, self.velocity, self.attack);
    }

    /// Apply input to the player character state, physics, etc.
    pub fn apply_input(&mut self, input: &Input) {
        if input.key_up {
            self.velocity.y -= self.velocity_delta
        }
        if input.key_down {
            self.velocity.y += self.velocity_delta
        }
        if input.key_left {
            self.velocity.x -= self.velocity_delta
        }
        if input.key_right {
            self.velocity.x += self.velocity_delta
        }
        self.attack = input.key_space;
    }

    // TODO: Uncomment and rework this later.
    // pub(crate) fn sync_level_collision(&mut self, col: &crate::level::Collision) {
    //     let p = Aabb::new(
    //         Point2::new(self.position.x - 96.0 / 4.0, self.position.y - 96.0 / 4.0),
    //         Point2::new(self.position.x + 96.0 / 4.0, self.position.y + 96.0 / 4.0),
    //     );
    //
    //     col.aabbs.iter().for_each(|aabb| {
    //         let cont = aabb.check_contact(&p);
    //         if cont.intersects {
    //             let offset = cont.min_trans * cont.penetration;
    //             self.position -= Vector2::new(offset.x, offset.y);
    //         }
    //     });
    // }
}

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("mesh error: {0}")]
    Mesh(#[from] MeshError),
}

pub const SPAWN_POSITION: (f32, f32) = (1.75, 1.75);

// --------------------------------------------------
// --- MESH ---
// --------------------------------------------------

// Represents a player character mesh.
pub struct Mesh {
    frame: f32,
    position: Point2<f32>,
    direction: Direction,

    texture_id: TextureID,
    pub textures: [video::TextureGroup; TEX_COUNT],

    pub vertex_buffer: Vec<wgpu::Buffer>,
    #[allow(dead_code)]
    vertex_count: Vec<u32>,

    pub index_buffer: Vec<wgpu::Buffer>,
    #[allow(dead_code)]
    pub index_count: Vec<u32>,

    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl Mesh {
    /// Creates a new player character mesh instance.
    pub fn new(video: &video::Video) -> Result<Self, MeshError> {
        // textures
        let texture_id = TextureID::Orc3Idle;
        let mut textures = Vec::with_capacity(TEX_ID_LOOKUP.len());
        for (idx, (sub_path, _, _)) in TEX_ID_LOOKUP.iter().enumerate() {
            let bytes = &std::fs::read(format!("{}{}", TEX_PATH_PREFIX, sub_path))
                .map_err(MeshError::ReadIO)?;
            let label = format!("{:?}", TextureID::from_index(idx));
            textures.push(video::TextureGroup::new(video, bytes, &label)?);
        }
        let textures = textures.try_into().map_err(|_| MeshError::ReadConvert)?;

        // geometry -- vertices
        let mut vertex_buffer = Vec::with_capacity(TEX_COLUMN_COUNTS.len());
        let mut vertex_count = Vec::with_capacity(TEX_COLUMN_COUNTS.len());
        for (idx, col_count) in TEX_COLUMN_COUNTS.iter().enumerate() {
            let (vs, vsc) = Self::build_vertices(*col_count);
            vertex_buffer.push(video.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("player_mesh_vertex_buffer_{}", idx)),
                    contents: bytemuck::cast_slice(&vs),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ));
            vertex_count.push(vsc);
        }

        // geometry -- indices
        let mut index_buffer = Vec::with_capacity(TEX_COLUMN_COUNTS.len());
        let mut index_count = Vec::with_capacity(TEX_COLUMN_COUNTS.len());
        for (idx, vsc) in vertex_count.iter().enumerate() {
            let (is, isc) = Self::build_indices(*vsc);
            index_buffer.push(video.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("player_mesh_index_buffer_{}", idx)),
                contents: bytemuck::cast_slice(&is),
                usage: wgpu::BufferUsages::INDEX,
            }));
            index_count.push(isc);
        }

        // WGPU buffer and bind group
        let buffer = video.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("player_mesh_buffer"),
            size: std::mem::size_of::<[crate::video::MatrixUniform; 1]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = video.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("player_mesh_bind_group"),
            layout: &video.bind_group_layouts[crate::video::BIND_GROUP_TRANSFORM as usize],
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
        });

        Ok(Self {
            frame: 0.0,
            position: Point2::new(0.0, 0.0),
            direction: Direction::Down,

            texture_id,
            textures,

            vertex_buffer,
            vertex_count,

            index_buffer,
            index_count,

            buffer,
            bind_group,
        })
    }

    /// Build vertices vector to be used to create a new vertex buffer.
    /// Internal helper for [`new`]. Short variable names are more
    /// readable here in this specific function
    fn build_vertices(col_count: u32) -> (Vec<crate::video::Vertex>, u32) {
        let s = TEX_TILE_SIZE as f32;
        let (w, h) = (col_count as f32 * s, TEX_ROW_COUNT as f32 * s);

        let vertex_count = VERTS_PER_TILE * TEX_ROW_COUNT * col_count;
        let mut vertices = Vec::with_capacity(vertex_count as usize);

        for x in 0..col_count {
            for y in 0..TEX_ROW_COUNT {
                let (x, y) = (x as f32, y as f32);
                vertices.push(crate::video::Vertex::new(
                    (-VERT_XZ_COORD, VERT_Y_COORD, -VERT_XZ_COORD).into(),
                    ((s * x) / w, (s * y) / h).into(),
                ));
                vertices.push(crate::video::Vertex::new(
                    (-VERT_XZ_COORD, VERT_Y_COORD, VERT_XZ_COORD).into(),
                    ((s * x) / w, (s * (y + 1.0)) / h).into(),
                ));
                vertices.push(crate::video::Vertex::new(
                    (VERT_XZ_COORD, VERT_Y_COORD, VERT_XZ_COORD).into(),
                    ((s * (x + 1.0)) / w, (s * (y + 1.0)) / h).into(),
                ));
                vertices.push(crate::video::Vertex::new(
                    (VERT_XZ_COORD, VERT_Y_COORD, -VERT_XZ_COORD).into(),
                    ((s * (x + 1.0)) / w, (s * y) / h).into(),
                ));
            }
        }

        (vertices, vertex_count)
    }

    /// Build indices vector to be used to create a new index buffer.
    /// Internal helper for [`new`].
    fn build_indices(vertex_count: u32) -> (Vec<u16>, u32) {
        let index_count = vertex_count * INDS_PER_TILE;
        let mut indices = Vec::with_capacity(index_count as usize);

        for i in 0..(vertex_count / VERTS_PER_TILE) as u16 {
            let t = TRI_INDS.iter().map(|x| *x + i * VERTS_PER_TILE as u16);
            indices.extend(t);
        }

        (indices, index_count)
    }

    /// Advances internal mesh state changes. Such as animation, etc.
    fn advance(&mut self, position: Point2<f32>, velocity: Vector2<f32>, attack: bool) {
        // 1. update internal state
        self.position = position;
        self.direction = Direction::from_velocity(velocity);

        // 2. proceed/progress tiles animation
        self.frame += self.get_anim_speed();
        if self.frame >= self.get_max_frame() {
            self.frame = 0.0;
        }

        // 3. pick correct texture
        let mut texture_id = if attack { TextureID::Orc3Attack } else { TextureID::Orc3Idle };
        if velocity.magnitude2() > WALK_THRESHOLD {
            texture_id = if attack { TextureID::Orc3WalkAttack } else { TextureID::Orc3Walk };
        }
        if self.texture_id != texture_id {
            self.texture_id = texture_id;
            self.frame = 0.0;
        }
    }

    /// Render mesh with its current given state based on provided video instance and render pass.
    pub fn render(&self, vid: &video::Video, rp: &mut wgpu::RenderPass) {
        rp.set_bind_group(1, &self.bind_group, &[]);
        rp.set_bind_group(2, &self.textures[self.texture_id.index()].bind_group, &[]);

        let b = self.get_buffer();
        rp.set_vertex_buffer(0, self.vertex_buffer[b].slice(..));
        rp.set_index_buffer(self.index_buffer[b].slice(..), wgpu::IndexFormat::Uint16);

        let m = video::MatrixUniform {
            matrix: cgmath::Matrix4::from_translation(
                (self.position.x, 0.0, self.position.y).into(),
            )
            .into(),
        };
        vid.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&m.matrix));

        let idx = (self.frame as u32 * VERTS_PER_TILE + self.get_texture_row()) * INDS_PER_TILE;
        rp.draw_indexed(idx..idx + 6, 0, 0..1);
    }

    /// Little helper function.
    fn get_anim_speed(&self) -> f32 {
        TEX_ID_LOOKUP[self.texture_id.index()].1 as f32 / 4.0 * ANIM_SPEED
    }

    /// Little helper function.
    fn get_max_frame(&self) -> f32 {
        TEX_ID_LOOKUP[self.texture_id.index()].1 as f32
    }

    /// Little helper function.
    fn get_buffer(&self) -> usize {
        TEX_ID_LOOKUP[self.texture_id.index()].2 as usize
    }

    /// Little helper function.
    fn get_texture_row(&self) -> u32 {
        match self.direction {
            Direction::Up => 1,
            Direction::Down => 0,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("read io error: {0}")]
    ReadIO(#[from] std::io::Error),

    #[error("read convert error")]
    ReadConvert,

    #[error("texture error: {0}")]
    Texture(#[from] crate::video::TextureError),
}

// TODO: add Orc1 and Orc2 textures, after that there will be no need in `#[allow(...)]` below.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq)]
enum TextureID {
    Orc3Attack,
    Orc3Death,
    Orc3Hurt,
    Orc3Idle,
    Orc3Run,
    Orc3RunAttack,
    Orc3Walk,
    Orc3WalkAttack,
}

impl TextureID {
    /// Given an enum instance, provide index based on it.
    fn index(&self) -> usize {
        match *self {
            Self::Orc3Attack => 0,
            Self::Orc3Death => 1,
            Self::Orc3Hurt => 2,
            Self::Orc3Idle => 3,
            Self::Orc3Run => 4,
            Self::Orc3RunAttack => 5,
            Self::Orc3Walk => 6,
            Self::Orc3WalkAttack => 7,
        }
    }

    /// Given index, provide an enum instance based on it.
    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Orc3Attack,
            1 => Self::Orc3Death,
            2 => Self::Orc3Hurt,
            3 => Self::Orc3Idle,
            4 => Self::Orc3Run,
            5 => Self::Orc3RunAttack,
            6 => Self::Orc3Walk,
            7 => Self::Orc3WalkAttack,
            _ => panic!("invalid texture index"),
        }
    }
}

/// Tuple is a) the texture image file path, b) columns count in the image,
/// and c) vertex/index buffer index to be used to draw it.
const TEX_ID_LOOKUP: [(&str, u32, u32); TEX_COUNT] = [
    ("attack/orc3_attack_full.png", 8, 0), // TextureID::Orc3Attack
    ("death/orc3_death_full.png", 8, 0),   // TextureID::Orc3Death
    ("hurt/orc3_hurt_full.png", 6, 1),     // TextureID::Orc3Hurt
    ("idle/orc3_idle_full.png", 4, 2),     // TextureID::Orc3Idle
    ("run/orc3_run_full.png", 8, 0),       // TextureID::Orc3Run
    ("run_attack/orc3_run_attack_full.png", 8, 0), // TextureID::Orc3RunAttack
    ("walk/orc3_walk_full.png", 6, 1),     // TextureID::Orc3Walk
    ("walk_attack/orc3_walk_attack_full.png", 6, 1), // TextureID::Orc3WalkAttack
];

const TEX_COUNT: usize = 8;
const TEX_PATH_PREFIX: &str = "./assets/orc/png/Orc3/orc3_";
const TEX_COLUMN_COUNTS: [u32; 3] = [8, 6, 4];
const TEX_ROW_COUNT: u32 = 4;
const TEX_TILE_SIZE: u32 = 64;

const VERTS_PER_TILE: u32 = 4;
const INDS_PER_TILE: u32 = 6;

/// Two triangles, every of 3 indices/vertices.
const TRI_INDS: [u16; 6] = [0, 2, 3, 0, 1, 2];

const VERT_XZ_COORD: f32 = 1.0;
const VERT_Y_COORD: f32 = -0.25;

const ANIM_SPEED: f32 = 0.15;
const WALK_THRESHOLD: f32 = 0.000_005;
