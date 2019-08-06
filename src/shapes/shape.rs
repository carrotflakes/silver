use super::super::vec3::Vec3;
use super::super::ray::Ray;

pub struct HitRec {
    pub time: f64,
    pub location: Vec3,
    pub normal: Vec3,
}

pub trait Shape {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec>;
}
