pub mod shape;
pub mod sphere;
pub mod triangle;

pub use shape::Shape;
pub use sphere::Sphere;
pub use triangle::Triangle;

#[derive(Clone)]
pub enum Basic {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Shape for Basic {
    fn hit(&self, ray: &crate::ray::Ray, t0: f64, t1: f64) -> Option<shape::HitRec> {
        match self {
            Basic::Sphere(sphere) => sphere.hit(ray, t0, t1),
            Basic::Triangle(triangle) => triangle.hit(ray, t0, t1),
        }
    }
}
