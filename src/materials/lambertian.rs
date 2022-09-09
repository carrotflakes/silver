use super::super::ray::Ray;
use super::super::vec3::Vec3;
use super::material::Material;

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
    fn ray(&self, _ray: &Ray, location: &Vec3, normal: &Vec3, _uv: [f64; 2]) -> Ray {
        Ray::new(*location, *normal + Vec3::random_in_unit_sphere(&mut rand::thread_rng()))
    }

    fn color(&self, color: &Vec3, _uv: [f64; 2]) -> Vec3 {
        color.hadamard(&self.albedo)
    }
}
