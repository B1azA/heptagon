use wgpu_glyph::ab_glyph;

pub struct Font {
    brush: wgpu_glyph::GlyphBrush<()>,
    pub staging_belt: wgpu::util::StagingBelt,
}

impl Font {
    pub fn from_bytes(bytes: Vec<u8>, device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let font = ab_glyph::FontArc::try_from_vec(bytes).unwrap();
        let glyph_brush = wgpu_glyph::GlyphBrushBuilder::using_font(font.clone())
            .build(device, format);

        let staging_belt = wgpu::util::StagingBelt::new(1024);
        
        Self {
            brush: glyph_brush,
            staging_belt,
        }
    }

    pub fn from_path(path: &str, device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let bytes: Vec<u8> = std::fs::read(path).unwrap();
        Self::from_bytes(bytes, device, format)
    }

    pub fn queue(&mut self, position: glam::Vec2, bounds: glam::Vec2, text: &str) {
        self.brush.queue(
            wgpu_glyph::Section {
                screen_position: (position.x, position.y),
                bounds: (bounds.x, bounds.y),
                text: vec![wgpu_glyph::Text::new(text)
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(40.0)],
                ..wgpu_glyph::Section::default()
            }
        );        
    }

    pub fn render(&mut self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, 
        view: &wgpu::TextureView, size: (u32, u32)) {
        
        self.brush.draw_queued(device, &mut self.staging_belt, encoder, view, size.0, size.1).unwrap();
        self.staging_belt.finish();
    }

    pub fn get_texture(&mut self, dimensions: (u32, u32),
        device: &wgpu::Device, queue: &wgpu::Queue, 
        label: &str) -> super::texture::Texture {
        
        let texture = super::texture::Texture::empty(device, queue, dimensions, label).unwrap();
        let view = texture.get_view();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Text Encoder"),
        }); 

        {
            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });
        }

        self.queue(glam::Vec2::new(10.0, 10.0), glam::Vec2::new(100.0, 100.0), "Hello");
        self.render(device, &mut encoder, &view, dimensions);
        queue.submit(Some(encoder.finish()));
        // async_std::task::block_on(self.staging_belt.recall());

        texture
    }

    // pub fn render_text(&mut self, font: &mut Font, size: (u32, u32)) {
        
    //     let output = self.surface.get_current_texture().unwrap();
    //     let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    //     let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //         label: Some("Render Encoder"),
    //     });

    //     {
    //         let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //             label: Some("Render Pass"),
    //             color_attachments: &[
    //                 wgpu::RenderPassColorAttachment {
    //                     view: &view,
    //                     resolve_target: None,
    //                     ops: wgpu::Operations {
    //                         load: wgpu::LoadOp::Clear(
    //                             wgpu::Color {
    //                                 r: 0.1,
    //                                 g: 0.2,
    //                                 b: 0.3,
    //                                 a: 1.0,
    //                             }
    //                         ),
    //                         store: true,
    //                     }
    //                 }
    //             ],
    //             depth_stencil_attachment: None,
    //         });
    //     }

    //     font.queue(glam::Vec2::new(10.0, 10.0), glam::Vec2::new(100.0, 100.0), "Hello");
    //     font.render(&self.device, &mut self.staging_belt, &mut encoder, &view, size);
    //     self.queue.submit(Some(encoder.finish()));
    //     output.present();
    //     async_std::task::block_on(self.staging_belt.recall());
    // }
}