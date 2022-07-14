use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexColor {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl VertexColor {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            position,
            color,
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
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
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexTex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl VertexTex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
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
    pub position: [f32; 3]
}

impl Vertex {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            position,
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
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

#[derive(Debug, Copy, Clone)]
pub struct Vertices<'a, T: VertexBufferLayout> {
    vertices: &'a [T],
}

impl<'a, T: VertexBufferLayout> Vertices<'a, T> {
    pub fn new(vertices: &'a [T]) -> Self {
        Self {
            vertices,
        }
    }

    pub fn vertices(&self) -> &'a [T] {
        &self.vertices
    }

    pub fn set_vertices(&mut self, vertices: &'a [T]) {
        self.vertices = vertices;
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn to_bytes(&self) -> &'a [u8] {
        unsafe {
            let bytes = (self.vertices as *const [T]) as *const u8;
            return std::slice::from_raw_parts(bytes, self.vertices.len() * std::mem::size_of::<T>());
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

    pub fn buffer_layout(&self) -> wgpu::VertexBufferLayout {
        T::buffer_layout()
    }
}

pub trait VertexBufferLayout {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}

impl VertexBufferLayout for VertexTex {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        VertexTex::desc()
    }
}

impl VertexBufferLayout for Vertex {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        Vertex::desc()
    }
}

impl VertexBufferLayout for VertexColor {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        VertexColor::desc()
    }
}