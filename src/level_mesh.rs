use crate::video::{TextureError, TextureGroup, Vertex};
use cgmath::{Point2, Point3};
use thiserror::Error;
use wgpu::{util::DeviceExt, Device, Queue};

pub(crate) struct LevelMesh {
    pub(crate) texture_group: TextureGroup,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
}

impl LevelMesh {
    pub(crate) fn new(device: &Device, queue: &Queue) -> Result<Self, LevelMeshError> {
        let texture_group = TextureGroup::new(
            device,
            queue,
            include_bytes!("../assets/dungeon/Dungeon_Tileset.png"),
        )?;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("level_mesh_vertex_buffer"),
            contents: bytemuck::cast_slice(&Self::build_vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("level_mesh_index_buffer"),
            contents: bytemuck::cast_slice(&Self::build_indices()),
            usage: wgpu::BufferUsages::INDEX,
        });

        Ok(Self { texture_group, vertex_buffer, index_buffer })
    }

    fn build_vertices() -> Vec<Vertex> {
        let mut vertices = vec![];
        for i in 0..10 {
            for j in 0..10 {
                let (x, y) = (i as f32, j as f32);
                vertices.push(Vertex::new(
                    Point3::new(-0.5, 0.0, -0.5),
                    Point2::new((16.0 * (x + 1.0)) / 160.0, (16.0 * (y + 1.0)) / 160.0),
                ));
                vertices.push(Vertex::new(
                    Point3::new(-0.5, 0.0, 0.5),
                    Point2::new((16.0 * (x + 1.0)) / 160.0, (16.0 * y) / 160.0),
                ));
                vertices.push(Vertex::new(
                    Point3::new(0.5, 0.0, 0.5),
                    Point2::new((16.0 * x) / 160.0, 16.0 * y / 160.0),
                ));
                vertices.push(Vertex::new(
                    Point3::new(0.5, 0.0, -0.5),
                    Point2::new((16.0 * x) / 160.0, (16.0 * (y + 1.0)) / 160.0),
                ));
            }
        }
        vertices
    }

    fn build_indices() -> Vec<u16> {
        let mut indices = vec![];
        for i in 0..(10 * 10) {
            indices.extend([0, 2, 3, 0, 1, 2].iter().map(|x| *x + i * 4));
        }
        indices
    }
}

#[derive(Error, Debug)]
pub enum LevelMeshError {
    #[error("texture error: {0}")]
    Texture(#[from] TextureError),
}
