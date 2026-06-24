use std::mem::size_of;
use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};

use crate::hallways::util;

pub const MAX_MATERIAL_ID: usize = 0x1FF;
const MAX_FRAMES: usize = 0x1000;

#[derive(Debug, Clone, Copy)]
pub enum WriteError {
    TooManyMaterials,
    TooManyFrames,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TextureRef {
    pub bucket: u16,
    pub layer: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Entry {
    num_frames: u32,
    speed: f32,
    offset: u32,
    color: util::Color,
    texture_addressing: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Data {
    entries: [Entry; MAX_MATERIAL_ID + 1],
    frames: [u32; MAX_FRAMES],
    next_free_frame: u32,
}

impl Data {
    pub fn create() -> Self {
        return Zeroable::zeroed();
    }

    pub fn write(
        &mut self,
        material_ix: u32,
        speed: f32,
        texture_refs: &[TextureRef],
        color: util::Color,
        texture_addressing: u32,
    ) -> Result<(), WriteError> {
        let material_id = material_ix as usize;

        if material_id > MAX_MATERIAL_ID {
            return Err(WriteError::TooManyMaterials);
        }
        if self.next_free_frame as usize + texture_refs.len() > MAX_FRAMES {
            return Err(WriteError::TooManyFrames);
        }
        let offset = self.next_free_frame;
        self.entries[material_id] = Entry {
            num_frames: texture_refs.len() as u32,
            speed,
            offset,
            color,
            texture_addressing,
        };
        for (i, &texture_ref) in texture_refs.iter().enumerate() {
            self.frames[offset as usize + i] = bytemuck::cast(texture_ref);
        }
        self.next_free_frame += texture_refs.len() as u32;
        return Ok(());
    }
}

pub struct MaterialIndex {
    buffer: wgpu::Buffer,
}

impl MaterialIndex {
    pub fn create(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size_of::<Data>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return Self { buffer };
    }

    pub fn write(&self, queue: &wgpu::Queue, data: &Data) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(data));
    }

    pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: NonZeroU64::new(size_of::<Data>() as u64),
            },
            count: None,
        }
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.buffer,
                offset: 0,
                size: NonZeroU64::new(size_of::<Data>() as u64),
            }),
        }
    }
}
