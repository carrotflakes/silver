pub mod sphere;
pub mod triangle;

pub use sphere::Sphere;
pub use triangle::Triangle;

use crate::{
    bbox::BBox,
    ray::Ray,
    vec3::{NormVec3, Vec3},
};

pub struct HitRec {
    pub time: f64,
    pub location: Vec3,
    pub normal: NormVec3,
    pub uv: [f64; 2],
    pub front: bool,
}

pub trait Shape {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec>;
    fn bbox(&self) -> BBox;
}

#[derive(Clone)]
pub enum Basic {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Shape for Basic {
    fn hit(&self, ray: &crate::ray::Ray, t0: f64, t1: f64) -> Option<HitRec> {
        match self {
            Basic::Sphere(sphere) => sphere.hit(ray, t0, t1),
            Basic::Triangle(triangle) => triangle.hit(ray, t0, t1),
        }
    }

    fn bbox(&self) -> BBox {
        match self {
            Basic::Sphere(sphere) => sphere.bbox(),
            Basic::Triangle(triangle) => triangle.bbox(),
        }
    }
}
