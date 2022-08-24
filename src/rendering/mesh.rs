use super::*;

pub struct Mesh<V: Vertex, I> {
    vertices: Vertices<V>,
    indices: Indices<I>,
}

impl<V: Vertex, I> Mesh<V, I> {
    pub fn new(vertices: Vertices<V>, indices: Indices<I>) -> Self {
        Self {
            vertices,
            indices,
        }
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        Vertices::<V>::vertex_buffer_layout()
    }

    pub fn vertex_buffer(&self, bundle: &super::bundle::Bundle) -> wgpu::Buffer {
        self.vertices.vertex_buffer(bundle)
    }

    pub fn index_buffer(&self, bundle: &super::bundle::Bundle) -> wgpu::Buffer {
        self.indices.index_buffer(bundle)
    }

    pub fn vertices(&self) -> &Vec<V> {
        self.vertices.vertices()
    }

    pub fn vertices_mut(&mut self) -> &mut Vec<V> {
        self.vertices.vertices_mut()
    }

    pub fn set_vertices(&mut self, vertices: Vec<V>) {
        self.vertices.set_vertices(vertices);
    }

    pub fn indices(&self) -> &Vec<I> {
        self.indices.indices()
    }

    pub fn indices_mut(&mut self) -> &mut Vec<I> {
        self.indices.indices_mut()
    }

    pub fn set_indices(&mut self, indices: Vec<I>) {
        self.indices.set_indices(indices);
    }

    pub fn mesh_buffer(&self, bundle: &super::bundle::Bundle) -> MeshBuffer {
        MeshBuffer {
            vertex_buffer: self.vertices.vertex_buffer(bundle),
            index_buffer: self.indices.index_buffer(bundle),
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