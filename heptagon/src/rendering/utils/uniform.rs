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

    pub fn update(&mut self, data: T) {
        self.data = data;
    }

    pub fn get_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform buffer"),
                contents: self.to_bytes(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        )
    }

    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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

    pub fn get_bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Self::get_bind_group_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.get_buffer(device).as_entire_binding(),
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
    pub fn new(model: glam::Mat4, view: glam::Mat4, proj: glam::Mat4) -> Self {
        Self {
            matrix: proj * view * model,
        }
    }

    pub fn to_bytes<'a>(&self) -> &'a [u8] {
        unsafe {
            let bytes = (self as *const Self) as *const u8;
            return std::slice::from_raw_parts(bytes, std::mem::size_of::<Self>());
        }
    }

    pub fn get_bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        let uniform = Uniform::new(*self);
        uniform.get_bind_group(device)
    }

    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        Uniform::<Self>::get_bind_group_layout(device)
    }
}