use std::mem::size_of;
use std::num::NonZeroU64;
use std::time::{SystemTime, UNIX_EPOCH};

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Data {
    clock: u32,
    _padding: [u32; 3],
}

pub struct Clock {
    buffer: wgpu::Buffer,
}

impl Data {
    pub fn new() -> Self {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        return Self {
            clock: millis as u32,
            _padding: [0; 3],
        };
    }
}

impl Clock {
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
            visibility: wgpu::ShaderStages::FRAGMENT,
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
