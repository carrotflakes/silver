use rand::Rng;

use crate::{
    bbox::BBox,
    ray::Ray,
    rng,
    shapes::{HitRec, Shape},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let d = b * b - 4.0 * a * c;
        if d > 0.0 {
            let root = d.sqrt();
            {
                let time = (-b - root) / (2.0 * a);
                if time < t1 && time > t0 {
                    let location = ray.at(time);
                    let normal = (location - self.center).normalize();
                    return Some(HitRec {
                        time,
                        location,
                        normal,
                        uv: get_sphere_uv(*normal),
                        front: true,
                    });
                }
            }
            {
                let time = (-b + root) / (2.0 * a);
                if time < t1 && time > t0 {
                    let location = ray.at(time);
                    let normal = (location - self.center).normalize();
                    return Some(HitRec {
                        time,
                        location,
                        normal,
                        uv: get_sphere_uv(*normal),
                        front: false,
                    });
                }
            }
        }
        None
    }

    fn bbox(&self) -> BBox {
        BBox::from_min_max(
            self.center - Vec3::new([self.radius, self.radius, self.radius]),
            self.center + Vec3::new([self.radius, self.radius, self.radius]),
        )
    }

    fn pdf_value(&self, ray: Ray) -> f64 {
        if let Some(_) = self.hit(&ray, 0.001, f64::INFINITY) {
            let cos_theta_max =
                (1.0 - self.radius.powi(2) / (self.center - ray.origin).norm_sqr()).sqrt();
            let solid_angle = std::f64::consts::TAU * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let direction = self.center - *origin;
        let distance_squared = direction.norm_sqr();
        let uvw = crate::onb::Onb::from_w(direction.normalize());
        uvw.local(random_to_sphere(self.radius, distance_squared))
    }
}

fn get_sphere_uv(p: Vec3) -> [f64; 2] {
    use std::f64::consts::PI;
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    [1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI]
}

fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = rng::with(|rng| rng.gen::<f64>());
    let r2 = rng::with(|rng| rng.gen::<f64>());
    let z = 1.0 + r2 * ((1.0 - radius.powi(2) / distance_squared).sqrt() - 1.0);
    let phi = std::f64::consts::TAU * r1;
    let x = phi.cos() * (1.0 - z.powi(2)).sqrt();
    let y = phi.sin() * (1.0 - z.powi(2)).sqrt();
    Vec3::new([x, y, z])
}
