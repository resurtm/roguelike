use crate::{
    dungeon_tiles::DungeonTile,
    level::Level,
    video::{TextureError, TextureGroup, Vertex},
};
use cgmath::{Point2, Point3};
use thiserror::Error;
use wgpu::{util::DeviceExt, Device, Queue};

pub struct LevelMesh {
    pub texture_group: TextureGroup,
    pub vertex_buffer: wgpu::Buffer,
    #[allow(dead_code)]
    pub num_vertices: u32,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl LevelMesh {
    pub fn new(device: &Device, queue: &Queue, level: &Level) -> Result<Self, LevelMeshError> {
        let texture_group = TextureGroup::new(
            device,
            queue,
            include_bytes!("../assets/dungeon/Dungeon_Tileset.png"),
        )?;

        let (vertices, num_vertices) = Self::build_vertices(level);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("level_mesh_vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let (indices, num_indices) = Self::build_indices(num_vertices);
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("level_mesh_index_buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Ok(Self { texture_group, vertex_buffer, num_vertices, index_buffer, num_indices })
    }

    fn build_vertices(level: &Level) -> (Vec<Vertex>, u32) {
        let tiles = DungeonTile::map_level_blocks_to_tiles(&level.blocks);
        let (mut vertices, mut num_vertices) = (vec![], 0);
        for (x, its) in tiles.iter().enumerate() {
            for (y, it) in its.iter().enumerate() {
                let (x, y) = (x as f32, y as f32);
                let (tx, ty) = DungeonTile::get_tex_pos(it).into();
                let (tx, ty) = (tx as f32, ty as f32);
                vertices.push(Vertex::new(
                    Point3::new(x - 0.5, 0.0, y - 0.5),
                    Point2::new((16.0 * tx) / 160.0, 16.0 * ty / 160.0),
                ));
                vertices.push(Vertex::new(
                    Point3::new(x - 0.5, 0.0, y + 0.5),
                    Point2::new((16.0 * tx) / 160.0, (16.0 * (ty + 1.0)) / 160.0),
                ));
                vertices.push(Vertex::new(
                    Point3::new(x + 0.5, 0.0, y + 0.5),
                    Point2::new((16.0 * (tx + 1.0)) / 160.0, (16.0 * (ty + 1.0)) / 160.0),
                ));
                vertices.push(Vertex::new(
                    Point3::new(x + 0.5, 0.0, y - 0.5),
                    Point2::new((16.0 * (tx + 1.0)) / 160.0, (16.0 * ty) / 160.0),
                ));
                num_vertices += 4;
            }
        }
        (vertices, num_vertices)
    }

    // fn build_vertices_prev() -> Vec<Vertex> {
    //     let mut vertices = vec![];
    //     for i in 0..10 {
    //         for j in 0..10 {
    //             let (x, y) = (i as f32, j as f32);
    //             vertices.push(Vertex::new(
    //                 Point3::new(-0.5, 0.0, -0.5),
    //                 Point2::new((16.0 * (x + 1.0)) / 160.0, (16.0 * (y + 1.0)) / 160.0),
    //             ));
    //             vertices.push(Vertex::new(
    //                 Point3::new(-0.5, 0.0, 0.5),
    //                 Point2::new((16.0 * (x + 1.0)) / 160.0, (16.0 * y) / 160.0),
    //             ));
    //             vertices.push(Vertex::new(
    //                 Point3::new(0.5, 0.0, 0.5),
    //                 Point2::new((16.0 * x) / 160.0, 16.0 * y / 160.0),
    //             ));
    //             vertices.push(Vertex::new(
    //                 Point3::new(0.5, 0.0, -0.5),
    //                 Point2::new((16.0 * x) / 160.0, (16.0 * (y + 1.0)) / 160.0),
    //             ));
    //         }
    //     }
    //     vertices
    // }

    fn build_indices(num_vertices: u32) -> (Vec<u16>, u32) {
        let (mut indices, mut num_indices) = (vec![], 0);
        for i in 0..num_vertices as u16 / 4 {
            indices.extend([0, 2, 3, 0, 1, 2].iter().map(|x| *x + i * 4));
            num_indices += 6;
        }
        (indices, num_indices)
    }
}

#[derive(Error, Debug)]
pub enum LevelMeshError {
    #[error("texture error: {0}")]
    Texture(#[from] TextureError),
}
