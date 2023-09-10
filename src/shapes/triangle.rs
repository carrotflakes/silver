use rand::Rng;

use crate::{
    bbox::BBox,
    ray::Ray,
    rng,
    shapes::{HitRec, Shape},
    vec3::{NormVec3, Vec3},
};

#[derive(Clone)]
pub struct Triangle<const BOTH_SIDE: bool = false>([Vec3; 3]);

impl<const BOTH_SIDE: bool> Triangle<BOTH_SIDE> {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Triangle([v0, v1, v2])
    }

    pub fn change_both_side<const NEW_BOTH_SIDE: bool>(self) -> Triangle<NEW_BOTH_SIDE> {
        Triangle(self.0)
    }
}

impl<const BOTH_SIDE: bool> Shape for Triangle<BOTH_SIDE> {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec> {
        if let Some((t, u, v, front)) =
            triangle_intersect(ray, &self.0[0], &self.0[1], &self.0[2], BOTH_SIDE)
        {
            if t0 < t && t < t1 {
                let location = ray.direction * t + ray.origin;
                let normal = if front {
                    triangle_norm(&self.0[0], &self.0[1], &self.0[2])
                } else {
                    triangle_norm(&self.0[0], &self.0[2], &self.0[1])
                };
                return Some(HitRec {
                    time: t,
                    location,
                    normal,
                    uv: [u, v],
                    front,
                });
            }
        }
        None
    }

    fn bbox(&self) -> BBox {
        BBox::from_min_max(
            Vec3::new([
                self.0[0].x().min(self.0[1].x()).min(self.0[2].x()),
                self.0[0].y().min(self.0[1].y()).min(self.0[2].y()),
                self.0[0].z().min(self.0[1].z()).min(self.0[2].z()),
            ]),
            Vec3::new([
                self.0[0].x().max(self.0[1].x()).max(self.0[2].x()),
                self.0[0].y().max(self.0[1].y()).max(self.0[2].y()),
                self.0[0].z().max(self.0[1].z()).max(self.0[2].z()),
            ]),
        )
    }

    fn pdf_value(&self, ray: Ray) -> f64 {
        if let Some(hr) = self.hit(&ray, 0.001, f64::INFINITY) {
            let area = 0.5
                * (self.0[1] - self.0[0])
                    .cross(&(self.0[2] - self.0[0]))
                    .norm();
            let distance_squared = hr.time.powi(2) * ray.direction.norm_sqr();
            let cosine = (ray.direction.dot(&hr.normal)).abs() / ray.direction.norm();
            distance_squared / (cosine.max(1e-8) * area)
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let u = rng::with(|rng| rng.gen_range(0.0..1.0));
        let v = rng::with(|rng| rng.gen_range(0.0..1.0));
        let random_point = if u + v > 1.0 {
            self.0[0] + (self.0[1] - self.0[0]) * (1.0 - u) + (self.0[2] - self.0[0]) * (1.0 - v)
        } else {
            self.0[0] + (self.0[1] - self.0[0]) * u + (self.0[2] - self.0[0]) * v
        };
        random_point - *origin
    }
}

pub fn triangle_norm(v0: &Vec3, v1: &Vec3, v2: &Vec3) -> NormVec3 {
    let e1 = *v1 - *v0;
    let e2 = *v2 - *v0;
    e1.cross(&e2).normalize()
}

// Tomas Moller
pub fn triangle_intersect(
    ray: &Ray,
    v0: &Vec3,
    v1: &Vec3,
    v2: &Vec3,
    reverse_side: bool,
) -> Option<(f64, f64, f64, bool)> {
    let e1 = *v1 - *v0;
    let e2 = *v2 - *v0;
    let pvec = ray.direction.cross(&e2);
    let det = e1.dot(&pvec);

    let qvec;
    let u;
    let v;
    if det > 1e-3 {
        let tvec = ray.origin - *v0;
        u = tvec.dot(&pvec);
        if u < 0.0 || u > det {
            return None;
        }

        qvec = tvec.cross(&e1);
        v = ray.direction.dot(&qvec);
        if v < 0.0 || u + v > det {
            return None;
        }

        let inv_det = 1.0 / det;
        let t = e2.dot(&qvec);
        return Some((t * inv_det, u * inv_det, v * inv_det, true));
    } else if reverse_side && det < -(1e-3) {
        let tvec = ray.origin - *v0;
        u = tvec.dot(&pvec);
        if u > 0.0 || u < det {
            return None;
        }

        qvec = tvec.cross(&e1);
        v = ray.direction.dot(&qvec);
        if v > 0.0 || u + v < det {
            return None;
        }

        let inv_det = 1.0 / det;
        let t = e2.dot(&qvec);
        return Some((t * inv_det, u * inv_det, v * inv_det, false));
    }
    return None;
}
