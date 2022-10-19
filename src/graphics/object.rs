pub struct Object {
    pos: glam::Vec3,
    rot: glam::Vec3,
    size: glam::Vec3,
}

impl Object {
    pub fn new(pos: glam::Vec3, rot: glam::Vec3, size: glam::Vec3) -> Self {
        Self {
            pos, rot, size,
        }
    }
}