use crate::{ray::Ray, vec3::Vec3};

pub struct BBox {
    min: Vec3,
    max: Vec3,
}

impl BBox {
    pub fn from_min_max(min: Vec3, max: Vec3) -> Self {
        BBox { min, max }
    }

    pub fn should_hit(&self, ray: &Ray) -> bool {
        (if 0.0 <= ray.direction.x() {
            ray.origin.x() < self.max.x()
        } else {
            self.min.x() < ray.origin.x()
        }) && (if 0.0 <= ray.direction.y() {
            ray.origin.y() < self.max.y()
        } else {
            self.min.y() < ray.origin.y()
        }) && (if 0.0 <= ray.direction.z() {
            ray.origin.z() < self.max.z()
        } else {
            self.min.z() < ray.origin.z()
        })
    }
}
