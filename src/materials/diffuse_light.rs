use super::super::ray::Ray;
use super::super::vec3::Vec3;
use super::material::Material;

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
    fn ray(&self, _ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray {
        Ray::new(*location, *normal)
    }

    fn color(&self, _color: &Vec3) -> Vec3 {
        self.color
    }
}
