use crate::hallways::gpu::texture::OitAccum;
use crate::hallways::gpu::texture::OitReveal;

pub struct Composite {
    bind_group: wgpu::BindGroup,
}

impl Composite {
    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                OitAccum::bind_group_layout_entry(0),
                OitReveal::bind_group_layout_entry(1),
            ],
        });
    }

    pub fn create(
        device: &wgpu::Device,
        oit_accum_texture: &OitAccum,
        oit_reveal_texture: &OitReveal,
    ) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                OitAccum::bind_group_layout_entry(0),
                OitReveal::bind_group_layout_entry(1),
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                oit_accum_texture.bind_group_entry(0),
                oit_reveal_texture.bind_group_entry(1),
            ],
        });

        return Self { bind_group };
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        return &self.bind_group;
    }
}
