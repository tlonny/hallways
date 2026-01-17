use crate::gpu::bind_group::Portal;
use crate::gpu::bind_group::World;
use crate::gpu::buffer::uniform;
use crate::gpu::pipeline;
use crate::gpu::render_pass::world;
use crate::gpu::texture::Color;

use super::Level;

pub struct PortalFrameBuffer {
    pub color_texture: Color,
    pub bind_group: Portal,
    pub camera_buffer: uniform::Camera,
    pub world_bind_group: World,
}

impl Level {
    pub fn render_opaque<'a>(
        &'a self,
        render_pass: &mut world::Color<'a>,
        pipeline: &'a pipeline::level::Opaque,
        world_bind_group: &'a World,
    ) {
        render_pass.render_level(
            pipeline,
            &self.bind_group,
            world_bind_group,
            &self.vertex_buffer,
            &self.index_buffer,
        );
    }

    pub fn render_transparent<'a>(
        &'a self,
        render_pass: &mut world::Oit<'a>,
        pipeline: &'a pipeline::level::Transparent,
        world_bind_group: &'a World,
    ) {
        render_pass.render_level(
            pipeline,
            &self.bind_group,
            world_bind_group,
            &self.vertex_buffer,
            &self.index_buffer,
        );
    }

    pub fn render_portals<'a>(
        &'a self,
        render_pass: &mut world::Color<'a>,
        pipeline: &'a pipeline::Portal,
        world_bind_group: &'a World,
        portal_frame_buffers: &'a [PortalFrameBuffer],
    ) {
        for portal in self.portals() {
            render_pass.render_portal(
                pipeline,
                &portal_frame_buffers[portal.index].bind_group,
                world_bind_group,
                &portal.vertex_buffer,
                &portal.index_buffer,
            );
        }
    }
}
