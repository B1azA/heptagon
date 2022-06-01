pub struct Font {
    font: rusttype::Font<'static>,
}

impl Font {
    pub fn from_path(path: &str) -> Self {
        let bytes = std::fs::read(path).unwrap();
        let font = rusttype::Font::try_from_vec(bytes).expect("Error loading a font");
        Self {
            font,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let font = rusttype::Font::try_from_vec(bytes).expect("Error loading a font");
        Self {
            font,
        }
    }

    pub fn get_image(&self, text: &str, scale: f32, color: (u8, u8, u8)) -> image::DynamicImage {
        let scale = rusttype::Scale::uniform(scale);
        let v_metrics = self.font.v_metrics(scale);
        let glyphs: Vec<_> = self.font.layout(text, scale, rusttype::point(20.0, 20.0 + v_metrics.ascent)).collect();
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };
        let mut img = image::DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40);
        let image = img.as_mut_rgba8().unwrap();

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        image::Rgba([color.0, color.1, color.2, (v * 255.0) as u8]),
                    )
                });
            }
        }

        img
    }
}