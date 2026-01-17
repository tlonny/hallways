use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

use crate::util;

const LEVEL_VERTEX_SHADER_LOCATION_POSITION: u32 = 0;
const LEVEL_VERTEX_SHADER_LOCATION_DIFFUSE_UV: u32 = 1;
const LEVEL_VERTEX_SHADER_LOCATION_MATERIAL_IX: u32 = 2;
const LEVEL_VERTEX_SHADER_LOCATION_COLOR: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Data {
    pub position: Vec3,
    pub diffuse_uv: Vec2,
    pub material_ix: u32,
    pub color: util::Color,
}

pub struct Level {
    buffer: wgpu::Buffer,
    capacity: usize,
}

#[derive(Debug)]
pub enum WriteError {
    CapacityExceeded,
}

impl Level {
    pub fn create(device: &wgpu::Device, capacity: usize) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (capacity * std::mem::size_of::<Data>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return Self { buffer, capacity };
    }

    pub fn write(&mut self, queue: &wgpu::Queue, vertices: &[Data]) -> Result<(), WriteError> {
        if vertices.len() > self.capacity {
            return Err(WriteError::CapacityExceeded);
        }
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(vertices));
        return Ok(());
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        return &self.buffer;
    }

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Data>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::offset_of!(Data, position) as u64,
                    shader_location: LEVEL_VERTEX_SHADER_LOCATION_POSITION,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::offset_of!(Data, diffuse_uv) as u64,
                    shader_location: LEVEL_VERTEX_SHADER_LOCATION_DIFFUSE_UV,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    offset: std::mem::offset_of!(Data, material_ix) as u64,
                    shader_location: LEVEL_VERTEX_SHADER_LOCATION_MATERIAL_IX,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Unorm8x4,
                    offset: std::mem::offset_of!(Data, color) as u64,
                    shader_location: LEVEL_VERTEX_SHADER_LOCATION_COLOR,
                },
            ],
        };
    }
}
