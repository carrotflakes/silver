use super::super::ray::Ray;
use super::super::vec3::Vec3;

pub trait Material {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3, uv: [f64; 2]) -> Ray;
    fn color(&self, color: &Vec3, uv: [f64; 2]) -> Vec3;
}
