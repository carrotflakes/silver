use crate::bbox::BBox;
use crate::ray::Ray;
use crate::vec3::{NormVec3, Vec3};

pub struct HitRec {
    pub time: f64,
    pub location: Vec3,
    pub normal: NormVec3,
    pub uv: [f64; 2],
}

pub trait Shape {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec>;
    fn bbox(&self) -> BBox;
}
