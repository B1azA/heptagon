pub struct Camera {
    position: glam::Vec3,
    target: glam::Vec3,
    up: glam::Vec3,
}

impl Camera {
    pub fn new(position: glam::Vec3, target: glam::Vec3, up: glam::Vec3) -> Self {
        Self {
            position,
            target,
            up,
        }
    }

    pub fn get_target(&self) -> glam::Vec3 {
        self.target
    }

    pub fn set_target(&mut self, target: glam::Vec3) {
        self.target = target;
    }

    pub fn get_position(&self) -> glam::Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: glam::Vec3) {
        self.position = position;
    }

    pub fn get_view_mat(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position, self.position + self.target, self.up)
    }

    pub fn get_forward(&self) -> glam::Vec3 {
        self.target - self.position
    }

    pub fn get_right(&self) -> glam::Vec3 {
        self.get_forward().cross(self.up)
    }

    pub fn set_angles(&mut self, yaw: f32, pitch: f32) {
        self.target = glam::Vec3::new(yaw.cos(), pitch.sin(), yaw.sin());
    }

    // pub fn get_yaw(&self) -> f32 {
    //     self.target.x.acos()
    // }

    // pub fn get_pitch(&self) -> f32 {
    //     self.target.y.asin()
    // }
}