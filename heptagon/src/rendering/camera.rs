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
        self.direction = direction;
    }

    pub fn position(&self) -> glam::Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: glam::Vec3) {
        self.position = position;
    }

    pub fn up(&self) -> glam::Vec3 {
        self.up
    }

    pub fn set_up(&mut self, up: glam::Vec3) {
        self.up = up;
    }

    pub fn view_mat(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position, self.position + self.direction, self.up)
    }

    pub fn forward(&self) -> glam::Vec3 {
        self.direction.normalize()
    }

    pub fn right(&self) -> glam::Vec3 {
        self.direction.cross(self.up)
    }

    const SAFE_FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2 - 0.001;

    pub fn set_angles(&mut self, yaw: f32, pitch: f32) {
        let mut pitch = pitch;
        if pitch < -Self::SAFE_FRAC_PI_2 {
            pitch = -Self::SAFE_FRAC_PI_2;
        } else if pitch > Self::SAFE_FRAC_PI_2 {
            pitch = Self::SAFE_FRAC_PI_2;
        }

        self.direction = glam::Vec3::new(yaw.cos(), pitch.sin(), yaw.sin());
    }

    pub fn set_angles_mut(&mut self, yaw: &mut f32, pitch: &mut f32) {
        let pitch = pitch;
        if *pitch < -Self::SAFE_FRAC_PI_2 {
            *pitch = -Self::SAFE_FRAC_PI_2;
        } else if *pitch > Self::SAFE_FRAC_PI_2 {
            *pitch = Self::SAFE_FRAC_PI_2;
        }

        self.direction = glam::Vec3::new(yaw.cos(), pitch.sin(), yaw.sin());
    }

    pub fn set_target(&mut self, target: glam::Vec3) {
        self.direction = target - self.position;
    }

    pub fn shift(&mut self, forward: f32, right: f32, up: f32) {
        self.position += self.forward() * forward;
        self.position += self.right() * right;
        self.position += self.up() * up;
    }

    pub fn yaw(&self) -> f32 {
        -self.forward().x.acos()
    }

    pub fn pitch(&self) -> f32 {
        self.forward().y.asin()
    }
}