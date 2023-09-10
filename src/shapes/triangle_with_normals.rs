use rand::Rng;

use crate::{
    bbox::BBox,
    ray::Ray,
    rng,
    shapes::{HitRec, Shape},
    vec3::Vec3,
};

use super::triangle::triangle_intersect;

#[derive(Clone)]
pub struct TriangleWithNormals<const BOTH_SIDE: bool = false> {
    vertexes: [Vec3; 3],
    normals: [Vec3; 3],
}

impl<const BOTH_SIDE: bool> TriangleWithNormals<BOTH_SIDE> {
    pub fn new(vertexes: [Vec3; 3], normals: [Vec3; 3]) -> Self {
        Self { vertexes, normals }
    }

    pub fn change_both_side<const NEW_BOTH_SIDE: bool>(self) -> TriangleWithNormals<NEW_BOTH_SIDE> {
        TriangleWithNormals {
            vertexes: self.vertexes,
            normals: self.normals,
        }
    }
}

impl<const BOTH_SIDE: bool> Shape for TriangleWithNormals<BOTH_SIDE> {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec> {
        if let Some((t, u, v, front)) = triangle_intersect(
            ray,
            &self.vertexes[0],
            &self.vertexes[1],
            &self.vertexes[2],
            BOTH_SIDE,
        ) {
            if t0 < t && t < t1 {
                let location = ray.direction * t + ray.origin;
                let normal = if front {
                    self.normals[0] * (1.0 - u * v) + self.normals[1] * u + self.normals[2] * v
                } else {
                    -(self.normals[0] * (1.0 - u * v) + self.normals[1] * u + self.normals[2] * v)
                }
                .normalize();
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
                self.vertexes[0]
                    .x()
                    .min(self.vertexes[1].x())
                    .min(self.vertexes[2].x()),
                self.vertexes[0]
                    .y()
                    .min(self.vertexes[1].y())
                    .min(self.vertexes[2].y()),
                self.vertexes[0]
                    .z()
                    .min(self.vertexes[1].z())
                    .min(self.vertexes[2].z()),
            ]),
            Vec3::new([
                self.vertexes[0]
                    .x()
                    .max(self.vertexes[1].x())
                    .max(self.vertexes[2].x()),
                self.vertexes[0]
                    .y()
                    .max(self.vertexes[1].y())
                    .max(self.vertexes[2].y()),
                self.vertexes[0]
                    .z()
                    .max(self.vertexes[1].z())
                    .max(self.vertexes[2].z()),
            ]),
        )
    }

    fn pdf_value(&self, ray: Ray) -> f64 {
        if let Some(hr) = self.hit(&ray, 0.001, f64::INFINITY) {
            let area = 0.5
                * (self.vertexes[1] - self.vertexes[0])
                    .cross(&(self.vertexes[2] - self.vertexes[0]))
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
            self.vertexes[0]
                + (self.vertexes[1] - self.vertexes[0]) * (1.0 - u)
                + (self.vertexes[2] - self.vertexes[0]) * (1.0 - v)
        } else {
            self.vertexes[0]
                + (self.vertexes[1] - self.vertexes[0]) * u
                + (self.vertexes[2] - self.vertexes[0]) * v
        };
        random_point - *origin
    }
}
