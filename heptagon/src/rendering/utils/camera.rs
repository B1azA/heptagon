use wgpu::util::DeviceExt;
use crate::rendering::utils::uniform::Uniform;

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

    pub fn get_bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(self);
        let mut uniform = Uniform::new(camera_uniform);
        uniform.get_bind_group(device)
    }

    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = glam::Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);
        let gl_to_wgpu = glam::Mat4::from_cols_array(&OPENGL_TO_WGPU_MATRIX);
        return gl_to_wgpu * proj * view;
    }

    pub fn get_view_mat(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.eye, self.target, self.up)
    }

    pub fn get_projection_mat(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar)
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        Uniform::<CameraUniform>::get_bind_group_layout(device)
    }

    pub fn forward(&self) -> glam::Vec3 {
        self.target - self.eye
    }

    pub fn right(&self) -> glam::Vec3 {
        self.forward().cross(self.up)
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
    view: glam::Mat4,
    proj: glam::Mat4,
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view: glam::Mat4::IDENTITY,
            proj: glam::Mat4::IDENTITY,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view = camera.get_view_mat();
        self.proj = camera.get_projection_mat();
    }

    pub fn to_bytes<'a>(&self) -> &'a [u8] {
        unsafe {
            let bytes = (self as *const Self) as *const u8;
            return std::slice::from_raw_parts(bytes, std::mem::size_of::<Self>());
        }
    }
}