use crate::{
    ray::Ray,
    rng,
    vec3::{NormVec3, Vec3},
};

use super::{Material, RayResult};

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &NormVec3, _uv: [f64; 2]) -> RayResult {
        let ray_dir = ray.direction.normalize();
        let b = -(ray_dir.dot(normal)) * **normal;
        let f = self.fuzz * rng::with(|rng| Vec3::random_in_unit_sphere(rng));
        RayResult {
            emit: Vec3::ZERO,
            albedo: self.albedo,
            scattered: Some(Ray::new(*location, *ray_dir + 2.0 * b + f)),
            pdf: None,
        }
    }
}
