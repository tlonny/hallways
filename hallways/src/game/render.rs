use crate::gpu::buffer::uniform::{camera, clock};

use super::effect::{level, overlay};
use super::Game;

impl Game {
    pub fn render(&mut self) {
        self.clock_buffer.write(&self.queue, clock::Data::new());

        let output = self.surface.get_current_texture().unwrap();
        let color_view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());

        level::render(level::RenderParams {
            state_scene: &self.state_scene,
            kinematics: &self.state_kinematics,
            queue: &self.queue,
            encoder: &mut encoder,
            color_view: &color_view,
            depth_view: self.depth_texture.view(),
            oit_accum_texture: &self.oit_accum_texture,
            oit_reveal_texture: &self.oit_reveal_texture,
            camera_buffer: &self.camera_buffer,
            world_bind_group: &self.world_bind_group,
            portal_frame_buffers: &self.portal_frame_buffers,
            pipeline_level: &self.pipeline_level,
            pipeline_level_transparent: &self.pipeline_level_transparent,
            pipeline_portal: &self.pipeline_portal,
            pipeline_composite: &self.pipeline_composite,
            composite_bind_group: &self.composite_bind_group,
            cache: &mut self.cache,
            camera: camera::DataNewParams {
                position: self.state_kinematics.eye_position(),
                rotation: self.state_intent.rotation,
                clip_position: glam::Vec3::ZERO,
                clip_normal: glam::Vec3::ZERO,
                aspect_ratio: self.surface_config.width as f32 / self.surface_config.height as f32,
            },
        });

        overlay::render(overlay::RenderParams {
            queue: &self.queue,
            encoder: &mut encoder,
            color_view: &color_view,
            pipeline_overlay: &self.pipeline_overlay,
            overlay_bind_group: &self.overlay_bind_group,
            overlay_buffer: &self.overlay_buffer,
            overlay_vertex_buffer: &mut self.overlay_vertex_buffer,
        });

        self.queue.submit([encoder.finish()]);
        output.present();
    }
}
