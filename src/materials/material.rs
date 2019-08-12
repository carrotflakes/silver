use super::super::vec3::Vec3;
use super::super::ray::Ray;

pub trait Material {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray;
    fn color(&self, coler: &Vec3) -> Vec3;
}
