const INDEX_FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint32;
const INDEX_START: u32 = 0;

pub struct Index {
    buffer: wgpu::Buffer,
    count: u32,
}

impl Index {
    pub fn create(device: &wgpu::Device, capacity: usize) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (capacity * std::mem::size_of::<u32>()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return Self { buffer, count: 0 };
    }

    pub fn upload(&mut self, queue: &wgpu::Queue, indices: &[u32]) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(indices));
        self.count = indices.len() as u32;
    }

    pub fn range(&self) -> std::ops::Range<u32> {
        return INDEX_START..self.count;
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        return &self.buffer;
    }

    pub fn format() -> wgpu::IndexFormat {
        return INDEX_FORMAT;
    }
}
