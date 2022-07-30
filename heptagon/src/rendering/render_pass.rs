use super::RenderPipeline;

pub struct RenderPass<'a> {
    render_pass: wgpu::RenderPass<'a>,
    texture_pipeline: &'a RenderPipeline,
    text_pipeline: &'a RenderPipeline,
    texture_pipeline_instanced: &'a RenderPipeline,
}

impl<'a> RenderPass<'a> {
    pub fn begin(
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        texture_pipeline: &'a RenderPipeline,
        text_pipeline: &'a RenderPipeline,
        texture_pipeline_instanced: &'a RenderPipeline,
        background_color: [f64; 4],
    ) -> Self {
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: background_color[0],
                        g: background_color[1],
                        b: background_color[2],
                        a: background_color[3],
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        Self {
            render_pass,
            texture_pipeline,
            text_pipeline,
            texture_pipeline_instanced,
        }
    }

    pub fn run_render_bundles(&mut self, render_bundles: &'a [wgpu::RenderBundle]) {
        self.render_pass.execute_bundles(render_bundles.iter());
    }

    pub fn render_pass(&mut self) -> &mut wgpu::RenderPass<'a> {
        &mut self.render_pass
    }

    pub fn render_texture(
        &mut self,
        vertex_buffer_slice: wgpu::BufferSlice<'a>,
        index_buffer_slice: wgpu::BufferSlice<'a>,
        indices_range: std::ops::Range<u32>,
        texture_bind_group: &'a wgpu::BindGroup,
        mvp_bind_group: &'a wgpu::BindGroup,
    ) {
        self.render_pass
            .set_pipeline(self.texture_pipeline.render_pipeline());
        self.render_pass.set_bind_group(0, &texture_bind_group, &[]);
        self.render_pass.set_bind_group(1, &mvp_bind_group, &[]);
        self.render_pass.set_vertex_buffer(0, vertex_buffer_slice);
        self.render_pass
            .set_index_buffer(index_buffer_slice, wgpu::IndexFormat::Uint16);
        self.render_pass.draw_indexed(indices_range, 0, 0..1);
    }

    pub fn render_texture_instanced(
        &mut self,
        vertex_buffer_slice: wgpu::BufferSlice<'a>,
        index_buffer_slice: wgpu::BufferSlice<'a>,
        indices_range: std::ops::Range<u32>,
        texture_bind_group: &'a wgpu::BindGroup,
        vp_bind_group: &'a wgpu::BindGroup,
        instance_buffer_slice: wgpu::BufferSlice<'a>,
        instances_range: std::ops::Range<u32>,
    ) {
        self.render_pass.set_pipeline(&self.texture_pipeline_instanced.render_pipeline());
        self.render_pass.set_bind_group(0, texture_bind_group, &[]);
        self.render_pass.set_bind_group(1, vp_bind_group, &[]);
        self.render_pass.set_vertex_buffer(0, vertex_buffer_slice);
        self.render_pass.set_vertex_buffer(1, instance_buffer_slice);
        self.render_pass.set_index_buffer(index_buffer_slice, wgpu::IndexFormat::Uint16);
        // UPDATED!
        self.render_pass.draw_indexed(indices_range, 0, instances_range);
    }

    pub fn render_text(
        &mut self,
        vertex_buffer_slice: wgpu::BufferSlice<'a>,
        index_buffer_slice: wgpu::BufferSlice<'a>,
        indices_range: std::ops::Range<u32>,
        texture_bind_group: &'a wgpu::BindGroup,
        mvp_bind_group: &'a wgpu::BindGroup,
        color_bind_group: &'a wgpu::BindGroup,
    ) {
        self.render_pass
            .set_pipeline(self.text_pipeline.render_pipeline());
        self.render_pass.set_bind_group(0, &texture_bind_group, &[]);
        self.render_pass.set_bind_group(1, &mvp_bind_group, &[]);
        self.render_pass.set_bind_group(2, &color_bind_group, &[]);
        self.render_pass.set_vertex_buffer(0, vertex_buffer_slice);
        self.render_pass
            .set_index_buffer(index_buffer_slice, wgpu::IndexFormat::Uint16);
        self.render_pass.draw_indexed(indices_range, 0, 0..1);
    }

    pub fn end(self) {}
}
