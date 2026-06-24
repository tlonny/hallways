use bytemuck::{Pod, Zeroable};
use glam::Vec3;

const PORTAL_VERTEX_SHADER_LOCATION_POSITION: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Data {
    pub position: Vec3,
}

pub struct Portal {
    buffer: wgpu::Buffer,
    capacity: usize,
}

#[derive(Debug)]
pub enum WriteError {
    CapacityExceeded,
}

impl Portal {
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
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::offset_of!(Data, position) as u64,
                shader_location: PORTAL_VERTEX_SHADER_LOCATION_POSITION,
            }],
        };
    }
}
