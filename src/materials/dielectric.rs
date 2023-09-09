use rand::Rng;

use crate::{
    ray::Ray,
    rng,
    vec3::{NormVec3, Vec3},
};

use super::{Material, RayResult};

#[derive(Clone)]
pub struct Dielectric {
    ri: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Dielectric {
        Dielectric { ri }
    }
}

impl Material for Dielectric {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &NormVec3, _uv: [f64; 2]) -> RayResult {
        let b: Vec3 = -(ray.direction.dot(normal)) * **normal;
        let reflected: Vec3 = ray.direction + 2.0 * b;

        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(normal) > 0.0 {
            (
                -**normal,
                self.ri,
                self.ri * ray.direction.dot(normal) / ray.direction.norm(),
            )
        } else {
            (
                **normal,
                self.ri.recip(),
                -ray.direction.dot(normal) / ray.direction.norm(),
            )
        };

        let v = match refract(&-ray.direction, &outward_normal, ni_over_nt) {
            Option::Some(ref refracted)
                if rng::with(|rng| rng.gen_range(0.0..1.0)) >= schlick(cosine, self.ri) =>
            {
                *refracted
            }
            _ => reflected,
        };
        RayResult {
            emit: Vec3::ZERO,
            albedo: Vec3::new([1.0; 3]),
            scattered: Some(Ray::new(*location, v)),
            pdf: None,
        }
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = *v.normalize();
    let dt = uv.dot(n);
    let d = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if d > 0.0 {
        Option::Some(-ni_over_nt * (uv - *n * dt) - *n * d.sqrt())
    } else {
        Option::None
    }
}

#[inline(always)]
fn schlick(cosine: f64, ri: f64) -> f64 {
    let r0 = (1.0 - ri) / (1.0 + ri).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
