use super::render_pipeline::RenderPipeline;

pub struct RenderBundle<'a> {
    encoder: wgpu::RenderBundleEncoder<'a>,
    texture_pipeline: &'a RenderPipeline,
    text_pipeline: &'a RenderPipeline,
}

impl<'a> RenderBundle<'a> {
    pub fn begin(
        device: &'a wgpu::Device,
        texture_pipeline: &'a RenderPipeline,
        text_pipeline: &'a RenderPipeline
    ) -> Self {
        let encoder = device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
            label: Some("Render Queue Encoder"),
            color_formats: &[Some(wgpu::TextureFormat::Bgra8UnormSrgb)],
            depth_stencil: None,
            sample_count: 1,
            multiview: None,
        });

        Self {
            encoder,
            texture_pipeline: texture_pipeline,
            text_pipeline: text_pipeline,
        }
    }

    pub fn render_texture(&mut self,
        vertex_buffer_slice: wgpu::BufferSlice<'a>,
        index_buffer_slice: wgpu::BufferSlice<'a>,
        indices: u32,
        texture_bind_group: &'a wgpu::BindGroup,
        mvp_bind_group: &'a wgpu::BindGroup
    ) {
        self.encoder.set_pipeline(self.texture_pipeline.render_pipeline());
        self.encoder.set_bind_group(0, &texture_bind_group, &[]);
        self.encoder.set_bind_group(1, &mvp_bind_group, &[]);
        self.encoder.set_vertex_buffer(0, vertex_buffer_slice);
        self.encoder.set_index_buffer(index_buffer_slice, wgpu::IndexFormat::Uint16);
        self.encoder.draw_indexed(0..indices, 0, 0..1);
    }

    pub fn render_text(&mut self,
        vertex_buffer_slice: wgpu::BufferSlice<'a>,
        index_buffer_slice: wgpu::BufferSlice<'a>,
        indices: u32,
        texture_bind_group: &'a wgpu::BindGroup,
        mvp_bind_group: &'a wgpu::BindGroup,
        color_bind_group: &'a wgpu::BindGroup,
    ) {
        self.encoder.set_pipeline(self.text_pipeline.render_pipeline());
        self.encoder.set_bind_group(0, &texture_bind_group, &[]);
        self.encoder.set_bind_group(1, &mvp_bind_group, &[]);
        self.encoder.set_bind_group(2, &color_bind_group, &[]);
        self.encoder.set_vertex_buffer(0, vertex_buffer_slice);
        self.encoder.set_index_buffer(index_buffer_slice, wgpu::IndexFormat::Uint16);
        self.encoder.draw_indexed(0..indices, 0, 0..1);
    }

    pub fn finish(self) -> wgpu::RenderBundle {
        self.encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("Render Queue"),
        })
    }
}