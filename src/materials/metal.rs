use super::material::Material;
use super::super::vec3::Vec3;
use super::super::ray::Ray;

pub struct Metal {
    fuzz: f64,
}

impl Metal {
    pub fn new(fuzz: f64) -> Metal {
        Metal {fuzz: fuzz}
    }
}

impl Material for Metal {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray {
        let b: Vec3 = -(ray.direction.dot(normal)) * *normal;
        let f: Vec3 = self.fuzz * Vec3::random_in_unit_sphere();
        Ray::new(*location, ray.direction + 2.0 * b + f)
    }

    fn color(&self, color: &Vec3) -> Vec3 {
        *color
    }
}
