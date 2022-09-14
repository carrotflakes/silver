use crate::{
    ray::Ray,
    rng,
    vec3::{NormVec3, Vec3},
};

use super::Material;

#[derive(Clone)]
pub struct ConstantMedium {
    neg_inv_density: f64,
    color: Vec3,
}

impl ConstantMedium {
    pub fn new(boundary: f64, color: Vec3) -> ConstantMedium {
        ConstantMedium {
            neg_inv_density: 1.0 / boundary,
            color,
        }
    }
}

impl Material for ConstantMedium {
    fn ray(&self, _ray: &Ray, location: &Vec3, _normal: &NormVec3, _uv: [f64; 2]) -> Ray {
        Ray::new(
            *location,
            *rng::with(|rng| Vec3::random_on_unit_sphere(rng)),
        )
    }

    fn color(&self, color: &Vec3, _uv: [f64; 2]) -> Vec3 {
        color.hadamard(&self.color)
    }

    fn volume(&self) -> Option<(f64, Vec3)> {
        Some((self.neg_inv_density, self.color))
    }
}
