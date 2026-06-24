use crate::gpu::bind_group::Composite;
use crate::gpu::bind_group::Overlay;
use crate::gpu::buffer::vertex;
use crate::gpu::pipeline;

const BIND_GROUP_INDEX: u32 = 0;
const VERTEX_BUFFER_SLOT: u32 = 0;
const VERTEX_START: u32 = 0;
const INSTANCE_START: u32 = 0;
const INSTANCE_COUNT: u32 = 1;
const FULLSCREEN_TRIANGLE_VERTEX_COUNT: u32 = 3;

pub struct Color<'a> {
    render_pass: wgpu::RenderPass<'a>,
}

impl<'a> Color<'a> {
    pub fn create(
        encoder: &'a mut wgpu::CommandEncoder,
        color_view: &'a wgpu::TextureView,
    ) -> Self {
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: color_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });

        return Self { render_pass };
    }

    pub fn render_composite(
        &mut self,
        pipeline: &'a pipeline::Composite,
        bind_group: &'a Composite,
    ) {
        self.render_pass.set_pipeline(pipeline.pipeline());
        self.render_pass
            .set_bind_group(BIND_GROUP_INDEX, bind_group.bind_group(), &[]);
        self.render_pass.draw(
            VERTEX_START..FULLSCREEN_TRIANGLE_VERTEX_COUNT,
            INSTANCE_START..INSTANCE_COUNT,
        );
    }

    pub fn render_overlay(
        &mut self,
        pipeline: &'a pipeline::Overlay,
        bind_group: &'a Overlay,
        vertex_buffer: &'a vertex::Overlay,
    ) {
        self.render_pass.set_pipeline(pipeline.pipeline());
        self.render_pass
            .set_bind_group(BIND_GROUP_INDEX, bind_group.bind_group(), &[]);
        self.render_pass
            .set_vertex_buffer(VERTEX_BUFFER_SLOT, vertex_buffer.buffer().slice(..));
        self.render_pass.draw(
            VERTEX_START..vertex_buffer.vertex_count(),
            INSTANCE_START..INSTANCE_COUNT,
        );
    }
}
