use crate::{
    ray::Ray,
    vec3::{NormVec3, Vec3},
};

use super::{Material, RayResult};

#[derive(Clone)]
pub struct WetGlass {
    center: Vec3,
}

impl WetGlass {
    pub fn new(center: Vec3) -> WetGlass {
        WetGlass { center }
    }
}

impl Material for WetGlass {
    fn ray(&self, ray: &Ray, location: &Vec3, _normal: &NormVec3, uv: [f64; 2]) -> RayResult {
        let x = (uv[0] - 0.25) * 4.0;
        let y = (uv[1] - 0.25) * 4.0;
        let r = x.hypot(y);
        let r = if r < 1.0 { r.powf(2.0) } else { 0.0 };
        let dir =
            (1.0 - r) * *ray.direction.normalize() + r * *(*location - self.center).normalize();

        RayResult {
            emit: Vec3::ZERO,
            albedo: Vec3::new([1.0; 3]),
            scattered: Some(Ray::new(*location, dir)),
            pdf: None,
        }
    }
}
