use crate::consts::WINDOW_SIZE;
use cgmath::{ortho, Matrix4, Point3, SquareMatrix, Vector3};
use image::{GenericImageView, ImageError};
use thiserror::Error;
use wgpu::util::DeviceExt;

pub(crate) struct Texture {
    #[allow(unused)]
    texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self, TextureError> {
        let image = image::load_from_memory(bytes)?;
        Ok(Self::create_internal(device, queue, &image, Some(label)))
    }

    #[allow(unused)]
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: &str,
    ) -> Self {
        Self::create_internal(device, queue, image, Some(label))
    }

    fn create_internal(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: Option<&str>,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: image.dimensions().0,
            height: image.dimensions().1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image.to_rgba8(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.width),
                rows_per_image: Some(size.height),
            },
            size,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });
        Self { texture, view, sampler }
    }
}

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("load from memory error: {0}")]
    LoadFromMemory(#[from] ImageError),
}

pub(crate) struct TextureGroup {
    #[allow(unused)]
    texture: Texture,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) bind_group: wgpu::BindGroup,
}

impl TextureGroup {
    pub(crate) fn new(device: &wgpu::Device, queue: &wgpu::Queue, bytes: &[u8]) -> Self {
        let texture = Texture::from_bytes(&device, &queue, bytes, "texture").unwrap();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });

        Self { texture, bind_group_layout, bind_group }
    }
}

pub(crate) struct Observer {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,

    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
}

impl Observer {
    pub(crate) fn default() -> Self {
        Self {
            eye: Point3::new(0.0, 1.0, 0.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::unit_z(),

            left: -((WINDOW_SIZE.0 / 2 / PIXELS_PER_TILE) as f32),
            right: (WINDOW_SIZE.0 / 2 / PIXELS_PER_TILE) as f32,
            bottom: -((WINDOW_SIZE.1 / 2 / PIXELS_PER_TILE) as f32),
            top: (WINDOW_SIZE.1 / 2 / PIXELS_PER_TILE) as f32,
            near: -1.0,
            far: 1.0,
        }
    }

    fn build_matrix(&self) -> Matrix4<f32> {
        let proj = ortho(self.left, self.right, self.bottom, self.top, self.near, self.far);
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

const PIXELS_PER_TILE: u32 = 64;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ObserverUniform {
    view_proj: [[f32; 4]; 4],
}

impl ObserverUniform {
    fn new() -> Self {
        Self { view_proj: Matrix4::identity().into() }
    }

    fn apply_observer(&mut self, observer: &Observer) {
        self.view_proj = observer.build_matrix().into();
    }
}

pub(crate) struct ObserverGroup {
    #[allow(unused)]
    observer: Observer,
    #[allow(unused)]
    uniform: ObserverUniform,
    #[allow(unused)]
    buffer: wgpu::Buffer,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) bind_group: wgpu::BindGroup,
}

impl ObserverGroup {
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        let observer = Observer::default();

        let mut uniform = ObserverUniform::new();
        uniform.apply_observer(&observer);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("observer_buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("observer_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("observer_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
        });

        Self { observer, uniform, buffer, bind_group_layout, bind_group }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub(crate) struct TexturedSquare {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) num_vertices: u32,

    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) num_indices: u32,
}

impl TexturedSquare {
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("textured_square_vertex_buffer"),
            contents: bytemuck::cast_slice(TEXTURED_SQUARE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = TEXTURED_SQUARE_VERTICES.len() as u32;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("textured_square_index_buffer"),
            contents: bytemuck::cast_slice(TEXTURED_SQUARE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = TEXTURED_SQUARE_INDICES.len() as u32;

        Self { vertex_buffer, num_vertices, index_buffer, num_indices }
    }
}

const TEXTURED_SQUARE_VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
    Vertex { position: [-1.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },
    Vertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [1.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
];

const TEXTURED_SQUARE_INDICES: &[u16] = &[0, 2, 3, 0, 1, 2];
