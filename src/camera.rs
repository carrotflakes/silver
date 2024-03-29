use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    diaphragm: f64,
}

impl Camera {
    pub fn new(
        origin: &Vec3,
        target: &Vec3,
        vup: &Vec3,
        vfov: f64,
        aspect: f64,
        diaphragm: f64,
        dof: f64,
    ) -> Camera {
        let half_h = (vfov / 2.0).tan();
        let half_w = aspect * half_h;
        let w = *(*origin - *target).normalize();
        let u = *(vup.cross(&w)).normalize();
        let v = w.cross(&u);
        Camera {
            origin: *origin,
            u: (2.0 * half_w * u) * dof,
            v: (2.0 * half_h * v) * dof,
            w: *origin - (half_w * u + half_h * v + w) * dof,
            diaphragm,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64, rng: &mut impl rand::Rng) -> Ray {
        if self.diaphragm == 0.0 {
            Ray::new(self.origin, self.w + self.u * u + self.v * v - self.origin)
        } else {
            let (fu, fv) = random_vec2_in_unit_circle(rng);
            let origin =
                self.origin + self.u * (fu * self.diaphragm) + self.v * (fv * self.diaphragm);
            Ray::new(origin, self.w + self.u * u + self.v * v - origin)
        }
    }
}

fn random_vec2_in_unit_circle(rng: &mut impl rand::Rng) -> (f64, f64) {
    loop {
        let (x, y) = (rng.gen::<f64>() * 2.0 - 1.0, rng.gen::<f64>() * 2.0 - 1.0);
        if x * x + y * y < 1.0 {
            return (x, y);
        }
    }
}
