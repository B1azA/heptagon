use wgpu::util::DeviceExt;

pub struct Camera {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    // controls
    pub speed: f32,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: [f32; 16] = 
    [  1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0, ];

impl Camera {
    pub fn new(eye: glam::Vec3, target: glam::Vec3, up: glam::Vec3, aspect: f32, fovy: f32, znear: f32, zfar: f32, speed: f32) -> Self {
        Self {
            eye,
            target,
            up,
            aspect,
            fovy,
            znear,
            zfar,
            speed,
        }
    }

    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = glam::Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);
        let gl_to_wgpu = glam::Mat4::from_cols_array(&OPENGL_TO_WGPU_MATRIX);
        return gl_to_wgpu * proj * view;
    }

    pub fn uniform(&self) -> CameraUniform {
        let mut uniform = CameraUniform::new();
        uniform.update_view_proj(&self);
        uniform
    }

    pub fn buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: self.uniform().to_bytes(),
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
                }
            ],
            label: Some("camera_bind_group_layout"),
        })
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
            label: Some("camera_binding_group")
        })
    }

    pub fn bind_group_with_layout(&self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.buffer(device).as_entire_binding(),
                }
            ],
            label: Some("camera_binding_group")
        })
    }

    pub fn shift(&mut self, direction: glam::Vec3) {
        let forward = self.target - self.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        if direction.z < 0.0 && forward_mag > self.speed {
            self.eye += forward_norm * self.speed;
        }
        if direction.z > 0.0 {
            self.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(self.up);
        let forward = self.target - self.eye;
        let forward_mag = forward.length();

        if direction.x > 0.0 {
            self.eye = self.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if direction.x < 0.0 {
            self.eye = self.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraUniform {
    view_proj: glam::Mat4,
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }

    pub fn to_bytes<'a>(&self) -> &'a [u8] {
        unsafe {
            let bytes = (&self.view_proj as *const glam::Mat4) as *const u8;
            return std::slice::from_raw_parts(bytes, 64);
        }
    }
}