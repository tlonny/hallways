use glam::UVec2;

use crate::hallways::gpu::buffer::storage;
use crate::hallways::gpu::texture::Array;
use crate::hallways::gpu::texture::Sampler;

const DIFFUSE_BINDING_START: u32 = 2;
const MATERIAL_INDEX_BINDING: u32 = DIFFUSE_BINDING_START + TEXTURE_BUCKETS.len() as u32;

#[derive(Clone, Copy)]
pub struct TextureBucket {
    pub dimensions: UVec2,
    pub layers: usize,
}

pub const TEXTURE_BUCKETS: [TextureBucket; 6] = [
    TextureBucket {
        dimensions: UVec2::new(0x800, 0x800),
        layers: 0x1,
    },
    TextureBucket {
        dimensions: UVec2::new(0x400, 0x400),
        layers: 0x4,
    },
    TextureBucket {
        dimensions: UVec2::new(0x200, 0x200),
        layers: 0x8,
    },
    TextureBucket {
        dimensions: UVec2::new(0x100, 0x100),
        layers: 0x20,
    },
    TextureBucket {
        dimensions: UVec2::new(0x80, 0x80),
        layers: 0x40,
    },
    TextureBucket {
        dimensions: UVec2::new(0x40, 0x40),
        layers: 0x100,
    },
];

pub struct Level {
    bind_group: wgpu::BindGroup,
}

impl Level {
    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                Sampler::bind_group_layout_entry(0),
                Sampler::bind_group_layout_entry(1),
                Array::bind_group_layout_entry(DIFFUSE_BINDING_START),
                Array::bind_group_layout_entry(DIFFUSE_BINDING_START + 1),
                Array::bind_group_layout_entry(DIFFUSE_BINDING_START + 2),
                Array::bind_group_layout_entry(DIFFUSE_BINDING_START + 3),
                Array::bind_group_layout_entry(DIFFUSE_BINDING_START + 4),
                Array::bind_group_layout_entry(DIFFUSE_BINDING_START + 5),
                storage::MaterialIndex::bind_group_layout_entry(MATERIAL_INDEX_BINDING),
            ],
        });
    }

    pub fn create(
        device: &wgpu::Device,
        diffuse: &[Array; TEXTURE_BUCKETS.len()],
        material_index: &storage::MaterialIndex,
    ) -> Self {
        let layout = Self::layout(device);
        let linear_sampler =
            Sampler::new(device, wgpu::AddressMode::Repeat, wgpu::FilterMode::Linear);
        let nearest_sampler =
            Sampler::new(device, wgpu::AddressMode::Repeat, wgpu::FilterMode::Nearest);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                linear_sampler.bind_group_entry(0),
                nearest_sampler.bind_group_entry(1),
                diffuse[0].bind_group_entry(DIFFUSE_BINDING_START),
                diffuse[1].bind_group_entry(DIFFUSE_BINDING_START + 1),
                diffuse[2].bind_group_entry(DIFFUSE_BINDING_START + 2),
                diffuse[3].bind_group_entry(DIFFUSE_BINDING_START + 3),
                diffuse[4].bind_group_entry(DIFFUSE_BINDING_START + 4),
                diffuse[5].bind_group_entry(DIFFUSE_BINDING_START + 5),
                material_index.bind_group_entry(MATERIAL_INDEX_BINDING),
            ],
        });

        return Self { bind_group };
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        return &self.bind_group;
    }
}
