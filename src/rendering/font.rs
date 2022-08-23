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

    pub fn glyph_texture(&self, device: &wgpu::Device,
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

pub struct Atlas<K: Eq, V> {
    keys: Vec<K>,
    vals: Vec<V>,
}

impl<K: Eq, V> Atlas<K, V> {
    pub fn new() -> Self {
        Self {
            keys: vec![],
            vals: vec![],
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.keys.contains(key)
    }

    pub fn index(&self, key: &K) -> usize {
        self.keys.iter().position(|x| x == key).unwrap()
    }

    pub fn value(&self, index: usize) -> &V {
        &self.vals[index]
    }

    pub fn add(&mut self, key: K, value: V) -> usize {
        self.keys.push(key);
        self.vals.push(value);

        self.vals.len() - 1
    }
}