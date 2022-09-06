use crate::{
    bbox::BBox,
    ray::Ray,
    shapes::shape::{HitRec, Shape},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Triangle(Vec3, Vec3, Vec3);

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Triangle {
        Triangle(v0, v1, v2)
    }
}

impl Shape for Triangle {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec> {
        if let Some((t, _u, _v)) = triangle_intersect(ray, &self.0, &self.1, &self.2) {
            if t0 < t && t < t1 {
                let location = ray.direction * t + ray.origin;
                return Some(HitRec {
                    time: t,
                    location,
                    normal: triangle_norm(&self.0, &self.1, &self.2),
                });
            }
        }
        None
    }

    fn bbox(&self) -> BBox {
        BBox::from_min_max(
            Vec3(
                self.0 .0.min(self.1 .0).min(self.2 .0),
                self.0 .1.min(self.1 .1).min(self.2 .1),
                self.0 .2.min(self.1 .2).min(self.2 .2),
            ),
            Vec3(
                self.0 .0.max(self.1 .0).max(self.2 .0),
                self.0 .1.max(self.1 .1).max(self.2 .1),
                self.0 .2.max(self.1 .2).max(self.2 .2),
            ),
        )
    }
}

fn triangle_norm(v0: &Vec3, v1: &Vec3, v2: &Vec3) -> Vec3 {
    let e1 = *v1 - *v0;
    let e2 = *v2 - *v0;
    e1.cross(&e2)
}

// Tomas Moller
fn triangle_intersect(ray: &Ray, v0: &Vec3, v1: &Vec3, v2: &Vec3) -> Option<(f64, f64, f64)> {
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
    } else if det < -(1e-3) {
        // let tvec = ray.origin - *v0;
        // u = tvec.dot(&pvec);
        // if u > 0.0 || u < det {
        //     return None;
        // }

        // qvec = tvec.cross(&e1);
        // v = ray.direction.dot(&qvec);
        // if v > 0.0 || u + v < det {
        //     return None;
        // }
        return None;
    } else {
        return None;
    }

    let inv_det = 1.0 / det;
    let t = e2.dot(&qvec);
    Some((t * inv_det, u * inv_det, v * inv_det))
}
