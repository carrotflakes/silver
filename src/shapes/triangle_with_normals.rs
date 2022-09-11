use crate::{
    bbox::BBox,
    ray::Ray,
    shapes::{HitRec, Shape},
    vec3::Vec3,
};

use super::triangle::triangle_intersect;

#[derive(Clone)]
pub struct TriangleWithNormals {
    vertexes: [Vec3; 3],
    normals: [Vec3; 3],
}

impl TriangleWithNormals {
    pub fn new(vertexes: [Vec3; 3], normals: [Vec3; 3]) -> Self {
        Self { vertexes, normals }
    }
}

impl Shape for TriangleWithNormals {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec> {
        if let Some((t, u, v, front)) = triangle_intersect(
            ray,
            &self.vertexes[0],
            &self.vertexes[1],
            &self.vertexes[2],
            true,
        ) {
            if t0 < t && t < t1 {
                let location = ray.direction * t + ray.origin;
                let normal = if front {
                    -(self.normals[0] * (1.0 - u * v) + self.normals[1] * u + self.normals[2] * v)
                } else {
                    self.normals[0] * (1.0 - u * v) + self.normals[1] * u + self.normals[2] * v
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
}
