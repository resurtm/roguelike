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

    uniform: ObserverUniform,
    buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl Observer {
    pub fn new(video: &crate::video::Video) -> Self {
        let uniform = ObserverUniform { view_proj: Matrix4::identity().into() };

        let buffer = video.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("observer_buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = video.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("observer_bind_group"),
            layout: &video.bind_group_layouts[crate::video::BIND_GROUP_OBSERVER as usize],
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
        });

        Self {
            eye: Point3::new(-5.0, 1.0, -5.0),
            target: Point3::new(-5.0, 0.0, -5.0),
            up: -Vector3::unit_z(),

            left: 0.0,
            right: 0.0,
            bottom: 0.0,
            top: 0.0,
            near: -10.0,
            far: 10.0,

            uniform,
            buffer,
            bind_group,
        }
    }

    pub fn handle_resize(&mut self, win_size: (u32, u32)) {
        let x = (win_size.0 / 2 / PIXELS_PER_TILE) as f32;
        let y = (win_size.1 / 2 / PIXELS_PER_TILE) as f32;

        self.left = -x;
        self.right = x;
        self.bottom = -y;
        self.top = y;
        self.near = -10.0;
        self.far = 10.0;
    }

    pub fn update(&mut self, video: &crate::video::Video) {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::ortho(self.left, self.right, self.bottom, self.top, self.near, self.far);
        let view_proj = OPENGL_TO_WGPU_MATRIX * proj * view;
        self.uniform.view_proj = view_proj.into();
        video.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ObserverUniform {
    view_proj: [[f32; 4]; 4],
}

const PIXELS_PER_TILE: u32 = 32 * 5;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
