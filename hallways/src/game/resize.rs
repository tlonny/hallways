use glam::Vec2;

use crate::hallways::gpu::bind_group::Composite;
use crate::hallways::gpu::bind_group::Portal;
use crate::hallways::gpu::bind_group::World;
use crate::hallways::gpu::buffer::uniform::{self, resolution};
use crate::hallways::gpu::texture::Color;
use crate::hallways::gpu::texture::Depth;
use crate::hallways::gpu::texture::OitAccum;
use crate::hallways::gpu::texture::OitReveal;
use crate::hallways::level::render::PortalFrameBuffer;

use super::Game;

pub const TARGET_WIDTH: f32 = 1280.0;

impl Game {
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);

        self.depth_texture = Depth::create(&self.device, width, height);
        self.oit_accum_texture = OitAccum::create(&self.device, (width, height));
        self.oit_reveal_texture = OitReveal::create(&self.device, (width, height));
        self.composite_bind_group = Composite::create(
            &self.device,
            &self.oit_accum_texture,
            &self.oit_reveal_texture,
        );
        self.camera_buffer = uniform::Camera::create(&self.device);
        self.world_bind_group =
            World::create(&self.device, &self.camera_buffer, &self.clock_buffer);
        self.portal_frame_buffers = std::array::from_fn(|_| {
            let color_texture =
                Color::create(&self.device, (width, height), self.surface_config.format);
            let bind_group = Portal::create(&self.device, &color_texture);
            let camera_buffer = uniform::Camera::create(&self.device);
            let world_bind_group = World::create(&self.device, &camera_buffer, &self.clock_buffer);
            return PortalFrameBuffer {
                color_texture,
                bind_group,
                camera_buffer,
                world_bind_group,
            };
        });

        let scale = (width as f32 / TARGET_WIDTH).floor().max(1.0);
        self.sprite_resolution = Vec2::new(width as f32 / scale, height as f32 / scale);
        self.sprite_resolution_buffer
            .write(&self.queue, resolution::Data::new(self.sprite_resolution));
    }
}
