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

    pub fn get_direction(&self) -> glam::Vec3 {
        self.direction
    }

    pub fn set_direction(&mut self, direction: glam::Vec3) {
        self.direction = direction.normalize();
    }

    pub fn get_position(&self) -> glam::Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: glam::Vec3) {
        self.position = position;
    }

    pub fn get_view_mat(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position, self.position + self.direction, self.up)
    }

    pub fn get_forward(&self) -> glam::Vec3 {
        self.direction - self.position
    }

    pub fn get_right(&self) -> glam::Vec3 {
        self.get_forward().cross(self.up)
    }

    pub fn set_angles(&mut self, yaw: f32, pitch: f32) {
        self.direction = glam::Vec3::new(yaw.cos(), pitch.sin(), yaw.sin());
    }

    pub fn offset_angles(&mut self, yaw: f32, pitch: f32) {
        let old_yaw = self.get_yaw();
        let old_pitch = self.get_pitch();
        self.direction += glam::Vec3::new((old_yaw + yaw).cos(), (pitch + old_pitch).sin(), (old_yaw + yaw).sin());
        self.direction = self.direction.normalize(); 
    }

    pub fn get_yaw(&self) -> f32 {
        -self.direction.x.acos()
    }

    pub fn get_pitch(&self) -> f32 {
        self.direction.y.asin()
    }
}