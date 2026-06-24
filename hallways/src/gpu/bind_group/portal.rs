use crate::hallways::gpu::texture::Color;
use crate::hallways::gpu::texture::Sampler;

pub struct Portal {
    bind_group: wgpu::BindGroup,
}

impl Portal {
    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                Sampler::bind_group_layout_entry(0),
                Color::bind_group_layout_entry(1),
            ],
        });
    }

    pub fn create(device: &wgpu::Device, color_texture: &Color) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                Sampler::bind_group_layout_entry(0),
                Color::bind_group_layout_entry(1),
            ],
        });
        let sampler = Sampler::new(
            device,
            wgpu::AddressMode::ClampToEdge,
            wgpu::FilterMode::Linear,
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                sampler.bind_group_entry(0),
                color_texture.bind_group_entry(1),
            ],
        });

        return Self { bind_group };
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        return &self.bind_group;
    }
}
