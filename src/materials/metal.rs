use super::super::ray::Ray;
use super::super::vec3::Vec3;
use super::material::Material;

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
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray {
        let b: Vec3 = -(ray.direction.dot(normal)) * *normal;
        let f: Vec3 = self.fuzz * Vec3::random_in_unit_sphere(&mut rand::thread_rng());
        Ray::new(*location, ray.direction + 2.0 * b + f)
    }

    fn color(&self, color: &Vec3) -> Vec3 {
        color.hadamard(&self.albedo)
    }
}
