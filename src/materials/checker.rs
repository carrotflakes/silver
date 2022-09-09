use super::super::ray::Ray;
use super::super::vec3::Vec3;
use super::material::Material;

#[derive(Clone)]
pub struct Checker<T: Material> {
    odd: Box<T>,
    even: Box<T>,
}

impl<T: Material> Checker<T> {
    pub fn new(odd: Box<T>, even: Box<T>) -> Self {
        Checker { odd, even }
    }
}

impl<T: Material> Material for Checker<T> {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3, uv: [f64; 2]) -> Ray {
        let [u, v] = uv;
        if ((u * 10.0).floor() as i32 + (v * 10.0).floor() as i32) % 2 == 0 {
            self.even.ray(ray, location, normal, uv)
        } else {
            self.odd.ray(ray, location, normal, uv)
        }
    }

    fn color(&self, color: &Vec3, uv: [f64; 2]) -> Vec3 {
        let [u, v] = uv;
        if ((u * 10.0).floor() as i32 + (v * 10.0).floor() as i32) % 2 == 0 {
            self.even.color(color, uv)
        } else {
            self.odd.color(color, uv)
        }
    }
}
