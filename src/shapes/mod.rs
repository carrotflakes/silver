pub mod edge;
pub mod sphere;
pub mod triangle;
pub mod triangle_with_normals;

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

    fn pdf_value(&self, ray: Ray) -> f64 {
        let _ = ray;
        0.0
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let _ = origin;
        Vec3::new([1.0, 0.0, 0.0])
    }
}

#[derive(Clone)]
pub enum Basic {
    Sphere(Sphere),
    Triangle(Triangle),
    Edge(edge::Edge),
}

impl Shape for Basic {
    fn hit(&self, ray: &crate::ray::Ray, t0: f64, t1: f64) -> Option<HitRec> {
        match self {
            Basic::Sphere(sphere) => sphere.hit(ray, t0, t1),
            Basic::Triangle(triangle) => triangle.hit(ray, t0, t1),
            Basic::Edge(edge) => edge.hit(ray, t0, t1),
        }
    }

    fn bbox(&self) -> BBox {
        match self {
            Basic::Sphere(sphere) => sphere.bbox(),
            Basic::Triangle(triangle) => triangle.bbox(),
            Basic::Edge(edge) => edge.bbox(),
        }
    }

    fn pdf_value(&self, ray: Ray) -> f64 {
        match self {
            Basic::Sphere(sphere) => sphere.pdf_value(ray),
            Basic::Triangle(triangle) => triangle.pdf_value(ray),
            Basic::Edge(edge) => edge.pdf_value(ray),
        }
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        match self {
            Basic::Sphere(sphere) => sphere.random(origin),
            Basic::Triangle(triangle) => triangle.random(origin),
            Basic::Edge(edge) => edge.random(origin),
        }
    }
}
