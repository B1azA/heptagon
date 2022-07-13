pub struct Model {
    scale: glam::Vec3,
    translation: glam::Vec3,
    rotation: glam::Quat,
}

impl Model {
    pub fn new(scale: glam::Vec3, translation: glam::Vec3, rotation: glam::Quat) -> Self {
        Self {
            scale,
            translation,
            rotation,
        }
    }

    pub fn get_model_mat(&self) -> glam::Mat4 {
        glam::Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    pub fn get_scale(&self) -> glam::Vec3 {
        self.scale
    }

    pub fn get_translation(&self) -> glam::Vec3 {
        self.translation
    }

    pub fn get_rotation(&self) -> glam::Quat {
        self.rotation
    }

    pub fn set_scale(&mut self, scale: glam::Vec3) {
        self.scale = scale;
    }

    pub fn set_translation(&mut self, translation: glam::Vec3) {
        self.translation = translation;
    }

    pub fn set_rotation(&mut self, rotation: glam::Quat) {
        self.rotation = rotation;
    }
    
}