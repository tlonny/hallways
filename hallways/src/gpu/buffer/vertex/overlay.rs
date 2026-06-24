use bytemuck::{Pod, Zeroable};
use glam::Vec2;

use crate::hallways::util;

const OVERLAY_VERTEX_SHADER_LOCATION_POSITION: u32 = 0;
const OVERLAY_VERTEX_SHADER_LOCATION_UV: u32 = 1;
const OVERLAY_VERTEX_SHADER_LOCATION_TEXTURE_IX: u32 = 2;
const OVERLAY_VERTEX_SHADER_LOCATION_COLOR: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Data {
    pub position: Vec2,
    pub uv: Vec2,
    pub texture_ix: u32,
    pub color: util::Color,
}

pub struct Overlay {
    buffer: wgpu::Buffer,
    capacity: usize,
    vertex_count: u32,
}

#[derive(Debug)]
pub enum WriteError {
    CapacityExceeded,
}

impl Overlay {
    pub fn create(device: &wgpu::Device, capacity: usize) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (capacity * std::mem::size_of::<Data>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return Self {
            buffer,
            capacity,
            vertex_count: 0,
        };
    }

    pub fn write(&mut self, queue: &wgpu::Queue, vertices: &[Data]) -> Result<(), WriteError> {
        if vertices.len() > self.capacity {
            return Err(WriteError::CapacityExceeded);
        }
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(vertices));
        self.vertex_count = vertices.len() as u32;
        return Ok(());
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        return &self.buffer;
    }

    pub fn vertex_count(&self) -> u32 {
        return self.vertex_count;
    }

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Data>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::offset_of!(Data, position) as u64,
                    shader_location: OVERLAY_VERTEX_SHADER_LOCATION_POSITION,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::offset_of!(Data, uv) as u64,
                    shader_location: OVERLAY_VERTEX_SHADER_LOCATION_UV,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    offset: std::mem::offset_of!(Data, texture_ix) as u64,
                    shader_location: OVERLAY_VERTEX_SHADER_LOCATION_TEXTURE_IX,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Unorm8x4,
                    offset: std::mem::offset_of!(Data, color) as u64,
                    shader_location: OVERLAY_VERTEX_SHADER_LOCATION_COLOR,
                },
            ],
        };
    }
}
