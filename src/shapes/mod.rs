pub mod edge;
pub mod sphere;
pub mod triangle;
pub mod triangle_with_normals;

pub use sphere::Sphere;
pub use triangle::Triangle;

use crate::{bbox::BBox, onb::Onb, ray::Ray, vec3::Vec3};

pub struct HitRec {
    pub time: f64,
    pub location: Vec3,
    pub normal: Onb,
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
    Triangle(Triangle<false>),
    TriangleBothSide(Triangle<true>),
    Edge(edge::Edge),
}

impl Basic {
    #[inline]
    fn as_ref(&self) -> &dyn Shape {
        match self {
            Basic::Sphere(sphere) => sphere,
            Basic::Triangle(triangle) => triangle,
            Basic::TriangleBothSide(triangle) => triangle,
            Basic::Edge(edge) => edge,
        }
    }
}

impl Shape for Basic {
    fn hit(&self, ray: &crate::ray::Ray, t0: f64, t1: f64) -> Option<HitRec> {
        self.as_ref().hit(ray, t0, t1)
    }

    fn bbox(&self) -> BBox {
        self.as_ref().bbox()
    }

    fn pdf_value(&self, ray: Ray) -> f64 {
        self.as_ref().pdf_value(ray)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        self.as_ref().random(origin)
    }
}
