use crate::{
    ray::Ray,
    rng,
    vec3::{NormVec3, Vec3},
};

use super::Material;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn ray(&self, _ray: &Ray, location: &Vec3, normal: &NormVec3, _uv: [f64; 2]) -> Ray {
        Ray::new(
            *location,
            **normal + rng::with(|rng| Vec3::random_in_unit_sphere(rng)),
        )
    }

    fn color(&self, color: &Vec3, _uv: [f64; 2]) -> Vec3 {
        color.hadamard(&self.albedo)
    }
}
