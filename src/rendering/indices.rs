use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct Indices<I> {
    indices: Vec<I>,
}

impl<I> Indices<I> {
    pub fn new(indices: Vec<I>) -> Self {
        Self {
            indices,
        }
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            let bytes = (self.indices.as_ref() as *const [I]) as *const u8;
            return std::slice::from_raw_parts(bytes, self.indices.len() * std::mem::size_of::<I>());
        }
    }

    pub fn index_buffer(&self, bundle: &super::bundle::Bundle) -> wgpu::Buffer {
        let index_buffer = bundle.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: self.to_bytes(),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        index_buffer
    }

    pub fn indices(&self) -> &Vec<I> {
        &self.indices
    }

    pub fn indices_mut(&mut self) -> &mut Vec<I> {
        &mut self.indices
    }

    pub fn set_indices(&mut self, indices: Vec<I>) {
        self.indices = indices;
    }
}