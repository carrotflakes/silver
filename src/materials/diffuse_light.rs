use crate::{
    ray::Ray,
    vec3::{NormVec3, Vec3},
};

use super::Material;

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
    fn ray(&self, _ray: &Ray, location: &Vec3, normal: &NormVec3, _uv: [f64; 2]) -> Ray {
        Ray::new(*location, **normal)
    }

    fn color(&self, _color: &Vec3, _uv: [f64; 2]) -> Vec3 {
        self.color
    }

    fn scatter(&self) -> bool {
        false
    }
}
