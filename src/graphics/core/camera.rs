/// Used for controlling camera movement and view matrix creation.
pub struct Camera {
    position: glam::Vec3,
    direction: glam::Vec3,
    up: glam::Vec3,
}

impl Camera {
    /// Creates a new Camera.
    pub fn new(position: glam::Vec3, direction: glam::Vec3) -> Self {
        Self {
            position,
            direction,
            up: glam::Vec3::Y,
        }
    }

    /// Returns its direction vector.
    pub fn direction(&self) -> glam::Vec3 {
        self.direction
    }

    /// Sets its direction vector.
    pub fn set_direction(&mut self, direction: glam::Vec3) {
        self.direction = direction;
    }

    /// Returns its position point.
    pub fn position(&self) -> glam::Vec3 {
        self.position
    }

    /// Sets its position point.
    pub fn set_position(&mut self, position: glam::Vec3) {
        self.position = position;
    }

    /// Returns its up vector.
    pub fn up(&self) -> glam::Vec3 {
        self.up
    }

    /// Sets its up vector.
    pub fn set_up(&mut self, up: glam::Vec3) {
        self.up = up;
    }

    /// Calculates a view matrix.
    pub fn view_mat(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position, self.position + self.direction, self.up)
    }

    /// Calculates its forward vector.
    pub fn forward(&self) -> glam::Vec3 {
        self.direction.normalize()
    }

    /// Calculates its right vector.
    pub fn right(&self) -> glam::Vec3 {
        self.direction.cross(self.up)
    }

    const SAFE_FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2 - 0.001;

    /// Sets its angles (yaw, pitch).
    pub fn set_angles(&mut self, yaw: f32, pitch: f32) {
        let mut pitch = pitch;
        if pitch < -Self::SAFE_FRAC_PI_2 {
            pitch = -Self::SAFE_FRAC_PI_2;
        } else if pitch > Self::SAFE_FRAC_PI_2 {
            pitch = Self::SAFE_FRAC_PI_2;
        }

        self.direction = glam::Vec3::new(yaw.cos(), pitch.sin(), yaw.sin());
    }

    /// Sets its angles and changes value of parameters if angle is out of bounds (-90..90 deg).
    pub fn set_angles_mut(&mut self, yaw: &mut f32, pitch: &mut f32) {
        let pitch = pitch;
        if *pitch < -Self::SAFE_FRAC_PI_2 {
            *pitch = -Self::SAFE_FRAC_PI_2;
        } else if *pitch > Self::SAFE_FRAC_PI_2 {
            *pitch = Self::SAFE_FRAC_PI_2;
        }

        self.direction = glam::Vec3::new(yaw.cos(), pitch.sin(), yaw.sin());
    }

    /// Sets the target of the camera.
    pub fn set_target(&mut self, target: glam::Vec3) {
        self.direction = target - self.position;
    }

    /// Moves with the camera.
    pub fn shift(&mut self, forward: f32, right: f32, up: f32) {
        self.position += self.forward() * forward;
        self.position += self.right() * right;
        self.position += self.up() * up;
    }

    /// Returns its yaw.
    pub fn yaw(&self) -> f32 {
        -self.forward().x.acos()
    }

    /// Returns its pitch.
    pub fn pitch(&self) -> f32 {
        self.forward().y.asin()
    }
}