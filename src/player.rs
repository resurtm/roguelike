use crate::{consts::START_POSITION, input::Input, video};
use cgmath::{Point2, Vector2};
use thiserror::Error;
use wgpu::util::DeviceExt;

pub(crate) struct PlayerOld {
    pub(crate) position: Point2<f64>,

    pub(crate) velocity: Vector2<f64>,
    velocity_delta: f64,
    velocity_max: f64,
    velocity_slowdown: f64,

    pub(crate) is_attack: bool,
}

impl PlayerOld {
    pub(crate) fn new() -> PlayerOld {
        PlayerOld {
            position: Point2::new(START_POSITION.0, START_POSITION.1),

            velocity: Vector2::new(0.0, 0.0),
            velocity_delta: 0.35,
            velocity_max: 6.5,
            velocity_slowdown: 0.92,

            is_attack: false,
        }
    }

    pub(crate) fn apply_input(&mut self, input: &Input) {
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

        self.is_attack = input.key_space;
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
    }

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

// --------------------------------------------------
// --- PLAYER ---
// --------------------------------------------------

pub struct Player {
    state: State,
    pub mesh: Mesh,
}

impl Player {
    pub fn new(video: &video::Video) -> Result<Self, PlayerError> {
        let state = State {};
        let mesh = Mesh::new(video)?;

        Ok(Self { state, mesh })
    }
}

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("mesh error: {0}")]
    Mesh(#[from] MeshError),
}

// --------------------------------------------------
// --- STATE ---
// --------------------------------------------------

pub struct State {}

// --------------------------------------------------
// --- MESH ---
// --------------------------------------------------

pub struct Mesh {
    pub textures: [video::TextureGroup; MESH_TEXTURE_COUNT],

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
    pub fn new(video: &video::Video) -> Result<Self, MeshError> {
        // textures
        let mut textures = vec![];
        for (id, sub_path, _, _) in MESH_TEXTURE_ID_LOOKUP.iter() {
            textures.push(video::TextureGroup::new(
                video,
                &std::fs::read(format!("{}{}", MESH_TEXTURE_PATH_PREFIX, sub_path))
                    .map_err(MeshError::ReadIO)?,
                &format!("{:?}", id),
            )?);
        }
        let textures = textures.try_into().map_err(|_| MeshError::ReadConvert)?;

        // geometry -- vertices
        let (mut vertex_buffer, mut vertex_count) = (vec![], vec![]);
        for (idx, &k) in [8, 6, 4].iter().enumerate() {
            let (vs, vsc) = Self::build_vertices(k);
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
        let (mut index_buffer, mut index_count) = (vec![], vec![]);
        for (idx, &vsc) in vertex_count.iter().enumerate() {
            let (is, isc) = Self::build_indices(vsc);
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
            layout: &video.bind_group_layouts[2],
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
        });

        Ok(Self {
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
    /// Internal helper for [`new`].
    fn build_vertices(k: u32) -> (Vec<crate::video::Vertex>, u32) {
        let s = MESH_TEXTURE_TILE_SIZE as f32;
        let (w, h) = (k as f32 * s, MESH_TEXTURE_HEIGHT as f32 * s);
        let (mut vertices, mut vertex_count) = (vec![], 0);
        for x in 0..k {
            for y in 0..MESH_TEXTURE_HEIGHT {
                let (x, y) = (x as f32, y as f32);
                vertices.push(crate::video::Vertex::new(
                    (-MESH_XZ_COORD, MESH_Y_COORD, -MESH_XZ_COORD).into(),
                    ((s * x) / w, (s * y) / h).into(),
                ));
                vertices.push(crate::video::Vertex::new(
                    (-MESH_XZ_COORD, MESH_Y_COORD, MESH_XZ_COORD).into(),
                    ((s * x) / w, (s * (y + 1.0)) / h).into(),
                ));
                vertices.push(crate::video::Vertex::new(
                    (MESH_XZ_COORD, MESH_Y_COORD, MESH_XZ_COORD).into(),
                    ((s * (x + 1.0)) / w, (s * (y + 1.0)) / h).into(),
                ));
                vertices.push(crate::video::Vertex::new(
                    (MESH_XZ_COORD, MESH_Y_COORD, -MESH_XZ_COORD).into(),
                    ((s * (x + 1.0)) / w, (s * y) / h).into(),
                ));
                vertex_count += MESH_VERTICES_PER_TILE;
            }
        }
        (vertices, vertex_count)
    }

    /// Build indices vector to be used to create a new index buffer.
    /// Internal helper for [`new`].
    fn build_indices(vertex_count: u32) -> (Vec<u16>, u32) {
        let (mut indices, mut index_count) = (vec![], 0);
        for i in 0..(vertex_count / MESH_VERTICES_PER_TILE) as u16 {
            let t = [0, 2, 3, 0, 1, 2].iter().map(|x| *x + i * MESH_VERTICES_PER_TILE as u16);
            indices.extend(t);
            index_count += MESH_INDICES_PER_TILE;
        }
        (indices, index_count)
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        render_pass.set_bind_group(0, &self.textures[3].bind_group, &[]);
        render_pass.set_bind_group(2, &self.bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer[2].slice(..));
        render_pass.set_index_buffer(self.index_buffer[2].slice(..), wgpu::IndexFormat::Uint16);

        let m = video::MatrixUniform {
            mat: cgmath::Matrix4::from_translation((-5.0f32, 0.0f32, -5.0f32).into()).into(),
        };
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&m.mat));

        render_pass.draw_indexed(0..6, 0, 0..1);
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

// TODO: add Orc1 and Orc2, and this will fix the Clippy disablement below
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
enum TextureID {
    Orc3Attack = 0, // iota like
    Orc3Death,
    Orc3Hurt,
    Orc3Idle,
    Orc3Run,
    Orc3RunAttack,
    Orc3Walk,
    Orc3WalkAttack,
}

// array of tuples, a tuple is (texture ID, filename, cols count / width, vertices offset)
const MESH_TEXTURE_ID_LOOKUP: [(TextureID, &str, u32, u32); MESH_TEXTURE_COUNT] = [
    (TextureID::Orc3Attack, "attack/orc3_attack_full.png", 8, 0),
    (TextureID::Orc3Death, "death/orc3_death_full.png", 8, 0),
    (TextureID::Orc3Hurt, "hurt/orc3_hurt_full.png", 6, 1),
    (TextureID::Orc3Idle, "idle/orc3_idle_full.png", 4, 2),
    (TextureID::Orc3Run, "run/orc3_run_full.png", 8, 0),
    (TextureID::Orc3RunAttack, "run_attack/orc3_run_attack_full.png", 8, 0),
    (TextureID::Orc3Walk, "walk/orc3_walk_full.png", 6, 1),
    (TextureID::Orc3WalkAttack, "walk_attack/orc3_walk_attack_full.png", 6, 1),
];
const MESH_TEXTURE_PATH_PREFIX: &str = "./assets/orc/png/Orc3/orc3_";
const MESH_TEXTURE_COUNT: usize = 8;

// note: width is different for every state texture
const MESH_TEXTURE_HEIGHT: u32 = 4;
const MESH_TEXTURE_TILE_SIZE: u32 = 64;
const MESH_VERTICES_PER_TILE: u32 = 4;
const MESH_INDICES_PER_TILE: u32 = 6;
const MESH_XZ_COORD: f32 = 1.0;
const MESH_Y_COORD: f32 = -0.25;
