use wgpu::util::DeviceExt;

pub struct Uniform<T> {
    data: T,
}

impl<T> Uniform<T> {
    pub fn new(data: T) -> Self {
        Self {
            data
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }

    pub fn buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform buffer"),
                contents: self.to_bytes(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        )
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("Uniform bind group layout"),
            }
        )
    }

    pub fn bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Self::bind_group_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.buffer(device).as_entire_binding(),
                }
            ],
            label: Some("Uniform bind group"),
        })
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            let bytes = (self as *const Self) as *const u8;
            return std::slice::from_raw_parts(bytes, std::mem::size_of::<Self>());
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat4Uniform {
    matrix: glam::Mat4,
}

impl Mat4Uniform {
    pub fn new(mat: glam::Mat4) -> Self {
        Self {
            matrix: mat,
        }
    }

    pub fn to_bytes<'a>(&self) -> &'a [u8] {
        unsafe {
            let bytes = (self as *const Self) as *const u8;
            return std::slice::from_raw_parts(bytes, std::mem::size_of::<Self>());
        }
    }

    pub fn bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        let uniform = Uniform::new(*self);
        uniform.bind_group(device)
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        Uniform::<Self>::bind_group_layout(device)
    }
}