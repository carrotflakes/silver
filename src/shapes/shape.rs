use crate::bbox::BBox;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRec {
    pub time: f64,
    pub location: Vec3,
    pub normal: Vec3,
}

pub trait Shape {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec>;
    fn bbox(&self) -> BBox;
}
