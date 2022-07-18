use super::texture::Texture;

pub struct Font {
    font: fontdue::Font,
}

impl Font {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let font = fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default()).unwrap();
        Self {
            font,
        }
    }

    pub fn from_path(path: &str) -> Self {
        let bytes: Vec<u8> = std::fs::read(path).unwrap();
        Self::from_bytes(bytes)
    }

    pub fn create_texture(&self, device: &wgpu::Device,
        queue: &wgpu::Queue, character: char,
        size: f32
    ) -> Texture {
        let (metrics, bitmap) = self.font.rasterize(character, size);

        let bytes: Vec<u8> = bitmap;
        let dimensions = (metrics.width as u32, metrics.height as u32);

        let texture = Texture::from_r8u_bytes(&bytes, dimensions, device, queue, "Font Texture").unwrap();
        texture
    }
}