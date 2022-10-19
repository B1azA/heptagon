use super::core::*;
use super::object::Object;

pub struct Image {
    texture: Texture,
    object: Object,
}

impl Image {
    pub fn new(texture: Texture, object: Object) -> Self {
        Self {
            texture, object,
        }
    }
}