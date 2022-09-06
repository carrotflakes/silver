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
        (if 0.0 <= ray.direction.0 {
            ray.origin.0 < self.max.0
        } else {
            self.min.0 < ray.origin.0
        }) && (if 0.0 <= ray.direction.1 {
            ray.origin.1 < self.max.1
        } else {
            self.min.1 < ray.origin.1
        }) && (if 0.0 <= ray.direction.2 {
            ray.origin.2 < self.max.2
        } else {
            self.min.2 < ray.origin.2
        })
    }
}
