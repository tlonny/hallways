use crate::gpu::bind_group::Level;
use crate::gpu::bind_group::Portal;
use crate::gpu::bind_group::World;
use crate::gpu::buffer::{self, vertex};
use crate::gpu::pipeline;
use crate::util;

const DRAW_BIND_GROUP_INDEX: u32 = 0;
const WORLD_BIND_GROUP_INDEX: u32 = 1;
const VERTEX_BUFFER_SLOT: u32 = 0;
const INSTANCE_START: u32 = 0;
const INSTANCE_COUNT: u32 = 1;

pub struct Color<'a> {
    render_pass: wgpu::RenderPass<'a>,
}

impl<'a> Color<'a> {
    pub fn create(
        encoder: &'a mut wgpu::CommandEncoder,
        color_view: &'a wgpu::TextureView,
        depth_view: &'a wgpu::TextureView,
    ) -> Self {
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: color_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(util::color::BLACK.into()),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });

        return Self { render_pass };
    }

    pub fn render_level(
        &mut self,
        pipeline: &'a pipeline::level::Opaque,
        bind_group: &'a Level,
        world_bind_group: &'a World,
        vertex_buffer: &'a vertex::Level,
        index_buffer: &'a buffer::Index,
    ) {
        self.render_pass.set_pipeline(pipeline.pipeline());
        self.render_pass
            .set_bind_group(DRAW_BIND_GROUP_INDEX, bind_group.bind_group(), &[]);
        self.render_pass
            .set_bind_group(WORLD_BIND_GROUP_INDEX, world_bind_group.bind_group(), &[]);
        self.render_pass
            .set_vertex_buffer(VERTEX_BUFFER_SLOT, vertex_buffer.buffer().slice(..));
        self.render_pass
            .set_index_buffer(index_buffer.buffer().slice(..), buffer::Index::format());
        self.render_pass
            .draw_indexed(index_buffer.range(), 0, INSTANCE_START..INSTANCE_COUNT);
    }

    pub fn render_portal(
        &mut self,
        pipeline: &'a pipeline::Portal,
        bind_group: &'a Portal,
        world_bind_group: &'a World,
        vertex_buffer: &'a vertex::Portal,
        index_buffer: &'a buffer::Index,
    ) {
        self.render_pass.set_pipeline(pipeline.pipeline());
        self.render_pass
            .set_bind_group(DRAW_BIND_GROUP_INDEX, bind_group.bind_group(), &[]);
        self.render_pass
            .set_bind_group(WORLD_BIND_GROUP_INDEX, world_bind_group.bind_group(), &[]);
        self.render_pass
            .set_vertex_buffer(VERTEX_BUFFER_SLOT, vertex_buffer.buffer().slice(..));
        self.render_pass
            .set_index_buffer(index_buffer.buffer().slice(..), buffer::Index::format());
        self.render_pass
            .draw_indexed(index_buffer.range(), 0, INSTANCE_START..INSTANCE_COUNT);
    }
}
