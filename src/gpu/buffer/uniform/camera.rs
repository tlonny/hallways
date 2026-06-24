use std::mem::size_of;
use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3, Vec4};

const PROJECTION_FOV_RADIANS: f32 = 75f32.to_radians();
const PROJECTION_NEAR: f32 = 0.05;
const PROJECTION_FAR: f32 = 1000.0;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Data {
    projection: Mat4,
    view: Mat4,
    clip_plane: Vec4,
}

#[derive(Clone, Copy)]
pub struct DataNewParams {
    pub position: Vec3,
    pub rotation: Vec2,
    pub clip_position: Vec3,
    pub clip_normal: Vec3,
    pub aspect_ratio: f32,
}

pub struct Camera {
    buffer: wgpu::Buffer,
}

impl Data {
    pub fn new(params: DataNewParams) -> Self {
        let projection = Mat4::perspective_rh(
            PROJECTION_FOV_RADIANS,
            params.aspect_ratio,
            PROJECTION_NEAR,
            PROJECTION_FAR,
        );

        let view = Mat4::from_rotation_x(-params.rotation.x)
            * Mat4::from_rotation_y(-params.rotation.y)
            * Mat4::from_translation(-params.position);

        let clip_plane = Vec4::new(
            params.clip_normal.x,
            params.clip_normal.y,
            params.clip_normal.z,
            -params.clip_normal.dot(params.clip_position),
        );

        return Self {
            projection,
            view,
            clip_plane,
        };
    }
}

impl Camera {
    pub fn create(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size_of::<Data>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return Self { buffer };
    }

    pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        return wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: NonZeroU64::new(size_of::<Data>() as u64),
            },
            count: None,
        };
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        return wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.buffer,
                offset: 0,
                size: NonZeroU64::new(size_of::<Data>() as u64),
            }),
        };
    }

    pub fn write(&self, queue: &wgpu::Queue, data: Data) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&data));
    }
}
