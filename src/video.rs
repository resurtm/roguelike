use crate::consts::WINDOW_SIZE;
use cgmath::{ortho, Matrix4, Point2, Point3, SquareMatrix, Vector3};
use image::{GenericImageView, ImageError};
use std::{iter, sync::Arc};
use thiserror::Error;
use wgpu::{util::DeviceExt, CreateSurfaceError, RequestDeviceError};
use winit::window::Window;

// --------------------------------------------------
// --- TEXTURE ---
// --------------------------------------------------

/// Represents and groups some important texture primitives/handles.
pub struct Texture {
    #[allow(unused)]
    texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    /// Create a new texture from provided bytes data.
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self, TextureError> {
        let image = image::load_from_memory(bytes)?;
        Ok(Self::create_internal(device, queue, &image, label))
    }

    /// Create a new texture from provided image.
    #[allow(unused)]
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: &str,
    ) -> Self {
        Self::create_internal(device, queue, image, label)
    }

    fn create_internal(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: &str,
    ) -> Self {
        println!("{:#?}", image.dimensions());
        let size = wgpu::Extent3d {
            width: image.dimensions().0,
            height: image.dimensions().1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("{}_texture", label)),
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

pub struct TextureGroup {
    #[allow(unused)]
    texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

impl TextureGroup {
    pub fn new(video: &Video, bytes: &[u8], label: &str) -> Result<Self, TextureError> {
        let texture = Texture::from_bytes(&video.device, &video.queue, bytes, label)?;

        let bind_group = video.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &video.bind_group_layouts[0],
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
            label: Some(&format!("{}_bind_group", label)),
        });

        Ok(Self { texture, bind_group })
    }
}

pub struct Observer {
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
    pub fn default() -> Self {
        Self {
            eye: Point3::new(0.0, 1.0, 0.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: -Vector3::unit_z(),

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

const PIXELS_PER_TILE: u32 = 32 * 3;

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

pub struct ObserverGroup {
    #[allow(unused)]
    observer: Observer,
    #[allow(unused)]
    uniform: ObserverUniform,
    #[allow(unused)]
    buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl ObserverGroup {
    pub fn new(video: &Video) -> Self {
        let observer = Observer::default();

        let mut uniform = ObserverUniform::new();
        uniform.apply_observer(&observer);

        let buffer = video.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("observer_buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = video.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("observer_bind_group"),
            layout: &video.bind_group_layouts[1],
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
        });

        Self { observer, uniform, buffer, bind_group }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(position: Point3<f32>, tex_coords: Point2<f32>) -> Self {
        Self { position: position.into(), tex_coords: tex_coords.into() }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
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

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MatrixUniform {
    pub mat: [[f32; 4]; 4],
}

pub struct Video<'a> {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    #[allow(dead_code)]
    surface: wgpu::Surface<'a>,
    #[allow(dead_code)]
    adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    pipeline: Option<wgpu::RenderPipeline>,
    pub bind_group_layouts: Vec<wgpu::BindGroupLayout>,
}

impl<'a> Video<'a> {
    pub async fn new(window: Arc<Window>) -> Result<Video<'a>, VideoError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let window_size = window.inner_size();
        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(VideoError::RequestAdapter())?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let mut bind_group_layouts = vec![];
        bind_group_layouts.push(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
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
            },
        ));
        bind_group_layouts.push(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
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
            },
        ));
        bind_group_layouts.push(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
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
                label: Some("level_mesh_bind_group_layout"),
            },
        ));

        Ok(Self {
            instance,
            adapter,
            surface,
            device,
            queue,
            config,
            pipeline: None,
            bind_group_layouts,
        })
    }

    pub fn handle_resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn get_pipeline(&mut self) -> &wgpu::RenderPipeline {
        self.pipeline.get_or_insert_with(|| {
            let shader = self.device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

            let pipeline_layout =
                self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("render_pipeline_layout"),
                    bind_group_layouts: &self.bind_group_layouts.iter().collect::<Vec<_>>(),
                    // &level_mesh.texture_group.bind_group_layout,
                    // &observer_group.bind_group_layout,
                    // &level_mesh_bind_group_layout,
                    push_constant_ranges: &[],
                });

            self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("render_pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.config.format,
                        // blend: Some(wgpu::BlendState::REPLACE),
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            })
        })
    }

    pub fn render(&mut self, scene: &crate::scene::Scene) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render_encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            // const BG_COLOR: (u8, u8, u8) = (37, 19, 26);
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(self.get_pipeline());
            render_pass.set_bind_group(1, &scene.observer.bind_group, &[]);

            // level
            render_pass.set_bind_group(0, &scene.level.mesh.texture.bind_group, &[]);
            render_pass.set_bind_group(2, &scene.level.mesh.bind_group, &[]);
            render_pass.set_vertex_buffer(0, scene.level.mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                scene.level.mesh.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            let m = MatrixUniform {
                mat: Matrix4::from_translation((-10.0f32, 0.0f32, -7.5f32).into()).into(),
            };
            self.queue.write_buffer(&scene.level.mesh.buffer, 0, bytemuck::cast_slice(&m.mat));
            render_pass.draw_indexed(0..scene.level.mesh.index_count, 0, 0..1);

            // player
            scene.player.mesh.render(self, &mut render_pass);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum VideoError {
    #[error("create surface error: {0}")]
    CreateSurface(#[from] CreateSurfaceError),

    #[error("request adapter error")]
    RequestAdapter(),

    #[error("request device error: {0}")]
    RequestDevice(#[from] RequestDeviceError),

    #[error("texture error: {0}")]
    Texture(#[from] TextureError),
}
