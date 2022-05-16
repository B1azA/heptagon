use wgpu::util::DeviceExt;

pub struct Indices<'a, T> {
    pub indices: &'a [T],
}

impl<'a, T> Indices<'a, T> {
    pub fn new(indices: &'a [T]) -> Self {
        Self {
            indices,
        }
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    pub fn to_bytes(&self) -> &'a [u8] {
        unsafe {
            let bytes = (self.indices as *const [T]) as *const u8;
            return std::slice::from_raw_parts(bytes, self.indices.len() * std::mem::size_of::<T>());
        }
    }

    pub fn to_index_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: self.to_bytes(),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        index_buffer
    }
}