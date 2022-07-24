use super::*;

pub struct Mesh<'a, V: VertexBufferLayout + 'a, I> {
    vertices: Vertices<'a, V>,
    indices: Indices<'a, I>,
}

impl<'a, V: VertexBufferLayout + 'a, I> Mesh<'a, V, I> {
    pub fn new(vertices: Vertices<'a, V>, indices: Indices<'a, I>) -> Self {
        Self {
            vertices,
            indices,
        }
    }

    pub fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'a> {
        Vertices::<V>::vertex_buffer_layout()
    }

    pub fn vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        self.vertices.vertex_buffer(device)
    }

    pub fn index_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        self.indices.index_buffer(device)
    }

    pub fn vertices(&self) -> &'a [V] {
        &self.vertices.vertices()
    }

    pub fn set_vertices(&mut self, vertices: &'a [V]) {
        self.vertices.set_vertices(vertices);
    }

    pub fn indices(&self) -> &'a [I] {
        &self.indices.indices()
    }

    pub fn set_indices(&mut self, indices: &'a [I]) {
        self.indices.set_indices(indices);
    }

    pub fn mesh_buffer(&self, device: &wgpu::Device) -> MeshBuffer {
        MeshBuffer {
            vertex_buffer: self.vertices.vertex_buffer(device),
            index_buffer: self.indices.index_buffer(device),
            index_count: self.indices.len() as u32,
        }
    }
}

pub struct MeshBuffer {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl MeshBuffer {
    pub fn vertex_buffer_slice(&self) -> wgpu::BufferSlice {
        self.vertex_buffer.slice(..)
    }

    pub fn index_buffer_slice(&self) -> wgpu::BufferSlice {
        self.index_buffer.slice(..)
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }
}