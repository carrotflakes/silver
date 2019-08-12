use super::material::Material;
use super::super::vec3::Vec3;
use super::super::ray::Ray;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {albedo: albedo}
    }
}

impl Material for Lambertian {
    fn ray(&self, _ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray {
        Ray::new(*location, *normal + Vec3::random_in_unit_sphere())
    }

    fn color(&self, color: &Vec3) -> Vec3 {
        color.hadamard(&self.albedo)
    }
}
