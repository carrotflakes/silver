use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone)]
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
        // It seems there is no difference in speed even if below exists.
        // && self.hit(ray)
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        hit_xy_plane(
            ray,
            [self.min.x(), self.max.x()],
            [self.min.y(), self.max.y()],
            if 0.0 <= ray.direction.z() {
                self.min.z()
            } else {
                self.max.z()
            },
        ) || hit_xz_plane(
            ray,
            [self.min.x(), self.max.x()],
            [self.min.z(), self.max.z()],
            if 0.0 <= ray.direction.y() {
                self.min.y()
            } else {
                self.max.y()
            },
        ) || hit_yz_plane(
            ray,
            [self.min.y(), self.max.y()],
            [self.min.z(), self.max.z()],
            if 0.0 <= ray.direction.x() {
                self.min.x()
            } else {
                self.max.x()
            },
        )
    }
}

const EPS: f64 = 1e-8;

fn hit_xy_plane(ray: &Ray, x: [f64; 2], y: [f64; 2], z: f64) -> bool {
    if ray.direction.z().abs() < EPS {
        return false;
    }
    let t = (z - ray.origin.z()) / ray.direction.z();
    let hit = ray.at(t);
    x[0] <= hit.x() && hit.x() <= x[1] && y[0] <= hit.y() && hit.y() <= y[1]
}

fn hit_xz_plane(ray: &Ray, x: [f64; 2], z: [f64; 2], y: f64) -> bool {
    if ray.direction.y().abs() < EPS {
        return false;
    }
    let t = (y - ray.origin.y()) / ray.direction.y();
    let hit = ray.at(t);
    x[0] <= hit.x() && hit.x() <= x[1] && z[0] <= hit.z() && hit.z() <= z[1]
}

fn hit_yz_plane(ray: &Ray, y: [f64; 2], z: [f64; 2], x: f64) -> bool {
    if ray.direction.x().abs() < EPS {
        return false;
    }
    let t = (x - ray.origin.x()) / ray.direction.x();
    let hit = ray.at(t);
    y[0] <= hit.y() && hit.y() <= y[1] && z[0] <= hit.z() && hit.z() <= z[1]
}
