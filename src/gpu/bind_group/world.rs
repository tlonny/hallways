use crate::gpu::buffer::uniform;

pub struct World {
    bind_group: wgpu::BindGroup,
}

impl World {
    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                uniform::Camera::bind_group_layout_entry(0),
                uniform::Clock::bind_group_layout_entry(1),
            ],
        });
    }

    pub fn create(device: &wgpu::Device, camera: &uniform::Camera, clock: &uniform::Clock) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                uniform::Camera::bind_group_layout_entry(0),
                uniform::Clock::bind_group_layout_entry(1),
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[camera.bind_group_entry(0), clock.bind_group_entry(1)],
        });

        return Self { bind_group };
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        return &self.bind_group;
    }
}
