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

    pub fn get_view_mat(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.eye, self.target, self.up)
    }

    pub fn get_projection_mat(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar)
    }

    pub fn forward(&self) -> glam::Vec3 {
        self.target - self.eye
    }

    pub fn right(&self) -> glam::Vec3 {
        self.forward().cross(self.up)
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