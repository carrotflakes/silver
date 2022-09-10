use crate::{
    ray::Ray,
    vec3::{NormVec3, Vec3},
};

use super::Material;

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
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &NormVec3, _uv: [f64; 2]) -> Ray {
        let b: Vec3 = -(ray.direction.dot(normal)) * **normal;
        let f: Vec3 = self.fuzz * Vec3::random_in_unit_sphere(&mut rand::thread_rng());
        Ray::new(*location, ray.direction + 2.0 * b + f)
    }

    fn color(&self, color: &Vec3, _uv: [f64; 2]) -> Vec3 {
        color.hadamard(&self.albedo)
    }
}
