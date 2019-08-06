use super::vec3::Vec3;
use super::ray::Ray;

pub struct Camera {
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(origin: &Vec3, target: &Vec3, vup: &Vec3, vfov: f64, aspect: f64) -> Camera {
        let half_h = (vfov / 2.0).tan();
        let half_w = aspect * half_h;
        let w = (*origin - *target).unit_vector();
        let u = (vup.cross(&w)).unit_vector();
        let v = w.cross(&u);
        Camera {
            origin: *origin,
            u: 2.0 * half_w * u,
            v: 2.0 * half_h * v,
            w: *origin - half_w * u - half_h * v - w
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.w + self.u * u + self.v * v - self.origin)
    }
}
