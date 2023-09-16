use crate::{onb::Onb, ray::Ray, vec3::Vec3};

use super::{Material, RayResult};

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
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Onb, uv: [f64; 2]) -> RayResult {
        let [u, v] = uv;
        if ((u * 10.0).floor() as i32 + (v * 10.0).floor() as i32) % 2 == 0 {
            self.even.ray(ray, location, normal, uv)
        } else {
            self.odd.ray(ray, location, normal, uv)
        }
    }
}
