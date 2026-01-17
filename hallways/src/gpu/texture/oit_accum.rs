pub struct OitAccum {
    view: wgpu::TextureView,
}

impl OitAccum {
    pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        return wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
    }

    pub fn create(device: &wgpu::Device, size: (u32, u32)) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        return Self {
            view: texture.create_view(&wgpu::TextureViewDescriptor::default()),
        };
    }

    pub fn view(&self) -> &wgpu::TextureView {
        return &self.view;
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        return wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(&self.view),
        };
    }
}
