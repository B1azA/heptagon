use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ColorVertex {
    pub position: glam::Vec3,
    pub color: glam::Vec3,
}

impl ColorVertex {
    pub fn new(position: glam::Vec3, color: glam::Vec3) -> Self {
        ColorVertex {
            position,
            color,
        }
    }
}

impl Vertex for ColorVertex {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ColorVertex>() as wgpu::BufferAddress,
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
#[derive(Copy, Clone, Debug)]
pub struct TextureVertex {
    pub position: glam::Vec3,
    pub tex_coords: glam::Vec2,
}

impl TextureVertex {
    pub fn new(position: glam::Vec3, tex_coords: glam::Vec2) -> Self {
        TextureVertex {
            position,
            tex_coords,
        }
    }
}

impl Vertex for TextureVertex {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextureVertex>() as wgpu::BufferAddress,
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
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PositionVertex {
    pub position: glam::Vec3,
}

impl PositionVertex {
    pub fn new(position: glam::Vec3) -> Self {
        PositionVertex {
            position,
        }
    }
}

impl Vertex for PositionVertex {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PositionVertex>() as wgpu::BufferAddress,
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

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ModelVertex {
    pub position: glam::Vec3,
    pub texture_coords: glam::Vec2,
    pub normal: glam::Vec3,
}

impl ModelVertex {
    pub fn new(position: glam::Vec3, texture_coords: glam::Vec2, normal: glam::Vec3) -> Self {
        Self {
            position,
            texture_coords,
            normal,
        }
    }
}

impl Vertex for ModelVertex {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
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
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct Vertices<V: Vertex> {
    vertices: Vec<V>,
}

impl<V: Vertex> Vertices<V> {
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

    pub fn vertex_buffer(&self, bundle: &super::bundle::Bundle) -> wgpu::Buffer {
        let vertex_buffer = bundle.device().create_buffer_init(
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

pub trait Vertex {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}