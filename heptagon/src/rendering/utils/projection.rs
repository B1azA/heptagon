pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
        }
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn fovy(&self) -> f32 {
        self.fovy
    }

    pub fn set_fovy(&mut self, fovy: f32) {
        self.fovy = fovy;
    }

    pub fn znear(&self) -> f32 {
        self.znear
    }

    pub fn set_znear(&mut self, znear: f32) {
        self.znear = znear;
    }

    pub fn zfar(&self) -> f32 {
        self.zfar
    }

    pub fn set_zfar(&mut self, zfar: f32) {
        self.zfar = zfar;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn projection_mat(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar)
    }
}