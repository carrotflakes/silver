use crate::{
    bbox::BBox,
    ray::Ray,
    shapes::{HitRec, Shape},
    vec3::{NormVec3, Vec3},
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
        if let Some((t, u, v, positive)) = triangle_intersect(ray, &self.0, &self.1, &self.2, true)
        {
            if t0 < t && t < t1 {
                let location = ray.direction * t + ray.origin;
                let normal = if positive {
                    triangle_norm(&self.0, &self.1, &self.2)
                } else {
                    triangle_norm(&self.0, &self.2, &self.1)
                };
                return Some(HitRec {
                    time: t,
                    location,
                    normal,
                    uv: [u, v],
                });
            }
        }
        None
    }

    fn bbox(&self) -> BBox {
        BBox::from_min_max(
            Vec3::new([
                self.0.x().min(self.1.x()).min(self.2.x()),
                self.0.y().min(self.1.y()).min(self.2.y()),
                self.0.z().min(self.1.z()).min(self.2.z()),
            ]),
            Vec3::new([
                self.0.x().max(self.1.x()).max(self.2.x()),
                self.0.y().max(self.1.y()).max(self.2.y()),
                self.0.z().max(self.1.z()).max(self.2.z()),
            ]),
        )
    }
}

fn triangle_norm(v0: &Vec3, v1: &Vec3, v2: &Vec3) -> NormVec3 {
    let e1 = *v1 - *v0;
    let e2 = *v2 - *v0;
    e1.cross(&e2).normalize()
}

// Tomas Moller
fn triangle_intersect(
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
