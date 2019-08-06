use super::vec3::Vec3;
use super::ray::Ray;

pub trait Material {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray;
    fn color(&self, coler: &Vec3) -> Vec3;
}

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
