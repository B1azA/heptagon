pub struct RenderQueue<'a> {
    encoder: wgpu::RenderBundleEncoder<'a>,
}

impl<'a> RenderQueue<'a> {
    pub fn begin(device: &'a wgpu::Device) -> Self {
        let encoder = device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
            label: Some("Render Queue Encoder"),
            color_formats: &[Some(wgpu::TextureFormat::Bgra8UnormSrgb)],
            depth_stencil: None,
            sample_count: 1,
            multiview: None,
        });

        Self {
            encoder,    
        }
    }

    pub fn render_texture(mut self,
        render_pipeline: &'a wgpu::RenderPipeline,
        vertex_buffer_slice: wgpu::BufferSlice<'a>,
        index_buffer_slice: wgpu::BufferSlice<'a>,
        indices: u32,
        texture_bind_group: &'a wgpu::BindGroup,
        mvp_bind_group: &'a wgpu::BindGroup) -> Self {

        self.encoder.set_pipeline(render_pipeline);
        self.encoder.set_bind_group(0, &texture_bind_group, &[]);
        self.encoder.set_bind_group(1, &mvp_bind_group, &[]);
        self.encoder.set_vertex_buffer(0, vertex_buffer_slice);
        self.encoder.set_index_buffer(index_buffer_slice, wgpu::IndexFormat::Uint16);
        self.encoder.draw_indexed(0..indices, 0, 0..1);

        self
    }

    pub fn finish(self) -> wgpu::RenderBundle {
        self.encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("Render Queue"),
        })
    }
}