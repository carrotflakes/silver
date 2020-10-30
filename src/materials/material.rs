use super::super::ray::Ray;
use super::super::vec3::Vec3;

pub trait Material {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray;
    fn color(&self, color: &Vec3) -> Vec3;
}
