use crate::{onb::Onb, ray::Ray, vec3::Vec3};

use super::{Material, RayResult};

#[derive(Clone)]
pub struct DiffuseLight {
    color: Vec3,
}

impl DiffuseLight {
    pub fn new(color: Vec3) -> DiffuseLight {
        DiffuseLight { color }
    }
}

impl Material for DiffuseLight {
    fn ray(&self, _ray: &Ray, _location: &Vec3, _normal: &Onb, _uv: [f64; 2]) -> RayResult {
        RayResult {
            emit: self.color,
            albedo: Vec3::ZERO,
            scattered: None,
            pdf: None,
        }
    }
}
