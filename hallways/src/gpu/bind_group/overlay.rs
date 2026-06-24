use crate::hallways::gpu::buffer::uniform;
use crate::hallways::gpu::texture::Array;
use crate::hallways::gpu::texture::Sampler;

pub struct Overlay {
    bind_group: wgpu::BindGroup,
}

impl Overlay {
    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                Sampler::bind_group_layout_entry(0),
                Array::bind_group_layout_entry(1),
                uniform::Resolution::bind_group_layout_entry(2),
            ],
        });
    }

    pub fn create(
        device: &wgpu::Device,
        diffuse: &Array,
        resolution_buffer: &uniform::Resolution,
    ) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                Sampler::bind_group_layout_entry(0),
                Array::bind_group_layout_entry(1),
                uniform::Resolution::bind_group_layout_entry(2),
            ],
        });

        let diffuse_sampler =
            Sampler::new(device, wgpu::AddressMode::Repeat, wgpu::FilterMode::Nearest);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                diffuse_sampler.bind_group_entry(0),
                diffuse.bind_group_entry(1),
                resolution_buffer.bind_group_entry(2),
            ],
        });

        return Self { bind_group };
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        return &self.bind_group;
    }
}
