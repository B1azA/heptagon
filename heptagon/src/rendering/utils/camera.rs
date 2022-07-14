pub struct Camera {
    position: glam::Vec3,
    direction: glam::Vec3,
    up: glam::Vec3,
}

impl Camera {
    pub fn new(position: glam::Vec3, direction: glam::Vec3) -> Self {
        Self {
            position,
            direction,
            up: glam::Vec3::Y,
        }
    }

    pub fn direction(&self) -> glam::Vec3 {
        self.direction
    }

    pub fn set_direction(&mut self, direction: glam::Vec3) {
        self.direction = direction.normalize();
    }

    pub fn position(&self) -> glam::Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: glam::Vec3) {
        self.position = position;
    }

    pub fn view_mat(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position, self.position + self.direction, self.up)
    }

    pub fn forward(&self) -> glam::Vec3 {
        self.direction - self.position
    }

    pub fn right(&self) -> glam::Vec3 {
        self.forward().cross(self.up)
    }

    pub fn set_angles(&mut self, yaw: f32, pitch: f32) {
        self.direction = glam::Vec3::new(yaw.cos(), pitch.sin(), yaw.sin());
    }

    pub fn offset_angles(&mut self, yaw: f32, pitch: f32) {
        let old_yaw = self.yaw();
        let old_pitch = self.pitch();
        self.direction += glam::Vec3::new((old_yaw + yaw).cos(), (pitch + old_pitch).sin(), (old_yaw + yaw).sin());
        self.direction = self.direction.normalize(); 
    }

    pub fn yaw(&self) -> f32 {
        -self.direction.x.acos()
    }

    pub fn pitch(&self) -> f32 {
        self.direction.y.asin()
    }
}