#[repr(C)]
#[derive(Copy, Clone)]
pub struct Instance {
    model: glam::Mat4,
}

impl Instance {
    pub fn new(model: glam::Mat4) -> Self {
        Self {
            model,
        }
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in
                // the shader.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            let bytes = (self as *const Self) as *const u8;
            return std::slice::from_raw_parts(bytes, std::mem::size_of::<Self>());
        }
    }

    pub fn model(&self) -> glam::Mat4 {
        self.model
    }

    pub fn set_model(&mut self, model: glam::Mat4) {
        self.model = model;
    }
}

pub struct Instances<I: super::Vertex> {
    instances: Vec<I>,
}

impl<I: super::Vertex> Instances<I> {
    pub fn new(instances: Vec<I>) -> Self {
        Self {
            instances
        }
    }

    pub fn instances(&self) -> &Vec<I> {
        &self.instances
    }

    pub fn instances_mut(&mut self) -> &mut Vec<I> {
        &mut self.instances
    }

    pub fn set_instances(&mut self, instances: Vec<I>) {
        self.instances = instances;
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            let bytes = (self.instances.as_ref() as *const [I]) as *const u8;
            return std::slice::from_raw_parts(bytes, self.instances.len() * std::mem::size_of::<I>());
        }
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        I::vertex_buffer_layout()
    }
}

impl super::Vertex for Instance {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        Self::vertex_buffer_layout()
    }
}