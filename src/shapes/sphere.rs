use super::super::vec3::Vec3;
use super::super::ray::Ray;
use super::shape::{HitRec, Shape};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {center: center, radius: radius}
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius.powi(2i32);
        let d = b * b - 4.0 * a * c;
        if d > 0.0 {
            let root = d.sqrt();
            {
                let time = (-b - root) / (2.0 * a);
                if time < t1 && time > t0 {
                    let location = ray.at(time);
                    return Option::Some(HitRec {
                        time: time,
                        location: location,
                        normal: (location - self.center).unit_vector()
                    });
                }
            }
            {
                let time = (-b + root) / (2.0 * a);
                if time < t1 && time > t0 {
                    let location = ray.at(time);
                    return Option::Some(HitRec {
                        time: time,
                        location: location,
                        normal: (location - self.center).unit_vector()
                    });
                }
            }
        }
        Option::None
    }
}
