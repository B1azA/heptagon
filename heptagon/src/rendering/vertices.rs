use wgpu::util::DeviceExt;

#[repr(C)]
pub struct VertexColor {
    pub position: glam::Vec3,
    pub color: glam::Vec3,
}

impl VertexColor {
    pub fn new(position: glam::Vec3, color: glam::Vec3) -> Self {
        Self {
            position,
            color,
        }
    }

    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexColor>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

#[repr(C)]
pub struct VertexTex {
    pub position: glam::Vec3,
    pub tex_coords: glam::Vec2,
}

impl VertexTex {
    pub fn new(position: glam::Vec3, tex_coords: glam::Vec2) -> Self {
        Self {
            position,
            tex_coords,
        }
    }

    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<VertexTex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: glam::Vec3,
}

impl Vertex {
    pub fn new(position: glam::Vec3) -> Self {
        Self {
            position,
        }
    }

    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ]
        }
    }
}

#[derive(Debug)]
pub struct Vertices<V: VertexBufferLayout> {
    vertices: Vec<V>,
}

impl<V: VertexBufferLayout> Vertices<V> {
    pub fn new(vertices: Vec<V>) -> Self {
        Self {
            vertices,
        }
    }

    pub fn vertices(&self) -> &Vec<V> {
        &self.vertices
    }

    pub fn vertices_mut(&mut self) -> &mut Vec<V> {
        &mut self.vertices
    }

    pub fn set_vertices(&mut self, vertices: Vec<V>) {
        self.vertices = vertices;
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            let bytes = (self.vertices.as_ref() as *const [V]) as *const u8;
            return std::slice::from_raw_parts(bytes, self.vertices.len() * std::mem::size_of::<V>());
        }
    }

    pub fn vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: self.to_bytes(),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        vertex_buffer
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        V::vertex_buffer_layout()
    }
}

pub trait VertexBufferLayout {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}

impl VertexBufferLayout for VertexTex {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        VertexTex::buffer_layout()
    }
}

impl VertexBufferLayout for Vertex {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        Vertex::buffer_layout()
    }
}

impl VertexBufferLayout for VertexColor {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        VertexColor::buffer_layout()
    }
}