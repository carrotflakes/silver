use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone, Debug)]
pub struct BBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BBox {
    pub fn from_min_max(min: Vec3, max: Vec3) -> Self {
        // Extend a bit
        // let max = max + Vec3::new([0.00001, 0.00001, 0.00001]);
        BBox { min, max }
    }

    pub fn merge(&self, other: &Self) -> Self {
        BBox {
            min: Vec3::new([
                self.min[0].min(other.min[0]),
                self.min[1].min(other.min[1]),
                self.min[2].min(other.min[2]),
            ]),
            max: Vec3::new([
                self.max[0].max(other.max[0]),
                self.max[1].max(other.max[1]),
                self.max[2].max(other.max[2]),
            ]),
        }
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

    pub fn hit_with_time(&self, ray: &Ray, tmin: f64, tmax: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let tmin = if t0 > tmin { t0 } else { tmin };
            let tmax = if t1 < tmax { t1 } else { tmax };
            // The following is slow. but why?
            // let tmin = t0.max(tmin);
            // let tmax = t1.min(tmax);

            // original: tmax <= tmin
            if tmax < tmin {
                return false;
            }
        }
        true
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
