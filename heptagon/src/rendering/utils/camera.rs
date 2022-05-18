pub struct Camera {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: [f32; 16] = 
    [  1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0, ];

impl Camera {
    pub fn new(eye: glam::Vec3, target: glam::Vec3, up: glam::Vec3, aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Self {
        Self {
            eye,
            target,
            up,
            aspect,
            fovy,
            znear,
            zfar,
        }
    }

    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = glam::Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);
        let gl_to_wgpu = glam::Mat4::from_cols_array(&OPENGL_TO_WGPU_MATRIX);
        return gl_to_wgpu * proj * view;
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