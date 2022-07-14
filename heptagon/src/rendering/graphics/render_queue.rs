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

    pub fn render_text(mut self, font: &mut crate::rendering::utils::text::Font,
        position: glam::Vec2,
        bounds: glam::Vec2,
        size: (u32, u32),
        text: &str,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        device: &wgpu::Device) -> Self {
        

        // color and scale
        font.brush.queue(
            wgpu_glyph::Section {
                screen_position: (position.x, position.y),
                bounds: (bounds.x, bounds.y),
                text: vec![wgpu_glyph::Text::new(text)
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(40.0)],
                ..wgpu_glyph::Section::default()
            }
        );

        font.brush.draw_queued(device, &mut font.staging_belt, encoder, view, size.0, size.1).unwrap();
        
        font.staging_belt.finish();

        self
    }

    pub fn finish(self) -> wgpu::RenderBundle {
        self.encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("Render Queue"),
        })
    }
}