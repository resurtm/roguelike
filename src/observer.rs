use crate::consts::WINDOW_SIZE;
use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};
use wgpu::util::DeviceExt;

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
            eye: Point3::new(-5.0, 1.0, -5.0),
            target: Point3::new(-5.0, 0.0, -5.0),
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
        let proj = cgmath::ortho(self.left, self.right, self.bottom, self.top, self.near, self.far);
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

const PIXELS_PER_TILE: u32 = 32 * 6;

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
    pub fn new(video: &crate::video::Video) -> Self {
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
            layout: &video.bind_group_layouts[0],
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
        });

        Self { observer, uniform, buffer, bind_group }
    }
}
