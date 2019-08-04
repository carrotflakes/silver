use super::vec3::Vec3;

pub struct PlainMat {
    color: Vec3,
}

impl PlainMat {
    pub fn new(color: Vec3) -> PlainMat {
        PlainMat {color: color}
    }

    pub fn color(&self) -> Vec3 {
        self.color
    }
}
