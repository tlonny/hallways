use crate::gpu::bind_group::Overlay;
use crate::gpu::buffer::vertex::{self, overlay};
use crate::gpu::pipeline;
use crate::gpu::render_pass::flat;

pub struct RenderParams<'a> {
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub color_view: &'a wgpu::TextureView,
    pub pipeline_overlay: &'a pipeline::Overlay,
    pub overlay_bind_group: &'a Overlay,
    pub overlay_buffer: &'a [overlay::Data],
    pub overlay_vertex_buffer: &'a mut vertex::Overlay,
}

pub fn render(params: RenderParams<'_>) {
    params
        .overlay_vertex_buffer
        .write(params.queue, params.overlay_buffer)
        .unwrap();

    let mut rp = flat::Color::create(params.encoder, params.color_view);
    rp.render_overlay(
        params.pipeline_overlay,
        params.overlay_bind_group,
        params.overlay_vertex_buffer,
    );
}
