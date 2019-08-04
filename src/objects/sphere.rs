use super::super::vec3::Vec3;
use super::super::ray::Ray;
use super::super::materials::PlainMat;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: PlainMat,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: PlainMat) -> Sphere {
        Sphere {center: center, radius: radius, material: material}
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius.powi(2i32);
        let d = b * b - 4.0 * a * c;
        d > 0.0
    }
}
