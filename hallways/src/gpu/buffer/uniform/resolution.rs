use std::mem::size_of;
use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};
use glam::Vec2;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Data {
    resolution: Vec2,
    _padding: [u32; 2],
}

pub struct Resolution {
    buffer: wgpu::Buffer,
}

impl Data {
    pub fn new(resolution: Vec2) -> Self {
        return Self {
            resolution,
            _padding: [0; 2],
        };
    }
}

impl Resolution {
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
            visibility: wgpu::ShaderStages::VERTEX,
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
