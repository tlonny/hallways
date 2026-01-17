use crate::gpu::bind_group::Level;
use crate::gpu::bind_group::World;
use crate::gpu::buffer::{self, vertex};
use crate::gpu::pipeline;
use crate::util;

const LEVEL_BIND_GROUP_INDEX: u32 = 0;
const WORLD_BIND_GROUP_INDEX: u32 = 1;
const VERTEX_BUFFER_SLOT: u32 = 0;
const INSTANCE_START: u32 = 0;
const INSTANCE_COUNT: u32 = 1;

pub struct Oit<'a> {
    render_pass: wgpu::RenderPass<'a>,
}

impl<'a> Oit<'a> {
    pub fn create(
        encoder: &'a mut wgpu::CommandEncoder,
        accum_view: &'a wgpu::TextureView,
        reveal_view: &'a wgpu::TextureView,
        depth_view: &'a wgpu::TextureView,
    ) -> Self {
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: accum_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(util::color::EMPTY.into()),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: reveal_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(util::color::WHITE.into()),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
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
        pipeline: &'a pipeline::level::Transparent,
        bind_group: &'a Level,
        world_bind_group: &'a World,
        vertex_buffer: &'a vertex::Level,
        index_buffer: &'a buffer::Index,
    ) {
        self.render_pass.set_pipeline(pipeline.pipeline());
        self.render_pass
            .set_bind_group(LEVEL_BIND_GROUP_INDEX, bind_group.bind_group(), &[]);
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
