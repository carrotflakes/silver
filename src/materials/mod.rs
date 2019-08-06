use super::vec3::Vec3;
use super::ray::Ray;

pub trait Material {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray;
    fn color(&self, coler: &Vec3) -> Vec3;
}

pub struct Metal {
    color: Vec3,
}

impl Metal {
    pub fn new(color: Vec3) -> Metal {
        Metal {color: color}
    }
}

impl Material for Metal {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray {
        let b: Vec3 = -(ray.direction.dot(normal)) * *normal;
        Ray::new(*location, ray.direction + 2.0 * b)
    }

    fn color(&self, color: &Vec3) -> Vec3 {
        *color * 0.8
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
        Ray::new(*location, *normal)
    }

    fn color(&self, color: &Vec3) -> Vec3 {
        color.hadamard(&self.albedo)
    }
}
