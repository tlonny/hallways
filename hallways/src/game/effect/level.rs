use crate::game::state::actor::Kinematics;
use crate::game::state::scene::Kind;
use crate::game::state::Scene;
use crate::gpu::bind_group::Composite;
use crate::gpu::bind_group::World;
use crate::gpu::buffer::uniform::{self, camera};
use crate::gpu::pipeline;
use crate::gpu::render_pass::flat;
use crate::gpu::render_pass::world::{Color, Oit};
use crate::gpu::texture::OitAccum;
use crate::gpu::texture::OitReveal;
use crate::level::cache::CacheEntry;
use crate::level::render::PortalFrameBuffer;
use crate::level::Cache;

pub struct RenderParams<'a> {
    pub state_scene: &'a Scene,
    pub kinematics: &'a Kinematics,
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub color_view: &'a wgpu::TextureView,
    pub depth_view: &'a wgpu::TextureView,
    pub oit_accum_texture: &'a OitAccum,
    pub oit_reveal_texture: &'a OitReveal,
    pub camera_buffer: &'a uniform::Camera,
    pub world_bind_group: &'a World,
    pub portal_frame_buffers: &'a [PortalFrameBuffer],
    pub pipeline_level: &'a pipeline::level::Opaque,
    pub pipeline_level_transparent: &'a pipeline::level::Transparent,
    pub pipeline_portal: &'a pipeline::Portal,
    pub pipeline_composite: &'a pipeline::Composite,
    pub composite_bind_group: &'a Composite,
    pub cache: &'a mut Cache,
    pub camera: camera::DataNewParams,
}

pub fn render(params: RenderParams<'_>) {
    if !matches!(
        params.state_scene.scene(),
        Kind::Simulation | Kind::MenuPause
    ) {
        return;
    }

    let Some(level_url) = params.kinematics.level_url.as_deref() else {
        return;
    };
    let Some(CacheEntry::Ready(level)) = params.cache.get(level_url) else {
        return;
    };

    for src_portal in level.portals() {
        let portal_frame_buffer = &params.portal_frame_buffers[src_portal.index];
        {
            // Clear stale portal renders
            let color_view = portal_frame_buffer.color_texture.view();
            let _render_pass = Color::create(params.encoder, color_view, params.depth_view);
        }

        let Some(link) = src_portal.link(params.cache) else {
            continue;
        };
        let target = src_portal.target.as_ref().unwrap();
        let Some(CacheEntry::Ready(dst_level)) = params.cache.get(&target.url) else {
            continue;
        };

        let src_geometry = &src_portal.geometry;
        let delta_yaw = link.delta_yaw();
        let eye_side = (src_geometry.center - params.camera.position)
            .dot(src_geometry.normal)
            .signum();
        let clip_normal = link.dst.normal * eye_side;
        let mut camera = params.camera;
        camera.position = link.transform_position(params.camera.position);
        camera.rotation.y += delta_yaw;
        camera.clip_position = link.dst.center;
        camera.clip_normal = clip_normal;

        portal_frame_buffer
            .camera_buffer
            .write(params.queue, camera::Data::new(camera));
        {
            let color_view = portal_frame_buffer.color_texture.view();
            let mut rp = Color::create(params.encoder, color_view, params.depth_view);
            dst_level.render_opaque(
                &mut rp,
                params.pipeline_level,
                &portal_frame_buffer.world_bind_group,
            );
        }
        {
            let mut rp = Oit::create(
                params.encoder,
                params.oit_accum_texture.view(),
                params.oit_reveal_texture.view(),
                params.depth_view,
            );
            dst_level.render_transparent(
                &mut rp,
                params.pipeline_level_transparent,
                &portal_frame_buffer.world_bind_group,
            );
        }
        {
            let mut rp =
                flat::Color::create(params.encoder, portal_frame_buffer.color_texture.view());
            rp.render_composite(params.pipeline_composite, params.composite_bind_group);
        }
    }

    {
        params
            .camera_buffer
            .write(params.queue, camera::Data::new(params.camera));
        let mut rp = Color::create(params.encoder, params.color_view, params.depth_view);
        level.render_opaque(&mut rp, params.pipeline_level, params.world_bind_group);
        level.render_portals(
            &mut rp,
            params.pipeline_portal,
            params.world_bind_group,
            params.portal_frame_buffers,
        );
    }
    {
        let mut rp = Oit::create(
            params.encoder,
            params.oit_accum_texture.view(),
            params.oit_reveal_texture.view(),
            params.depth_view,
        );
        level.render_transparent(
            &mut rp,
            params.pipeline_level_transparent,
            params.world_bind_group,
        );
    }
    {
        let mut rp = flat::Color::create(params.encoder, params.color_view);
        rp.render_composite(params.pipeline_composite, params.composite_bind_group);
    }
}
