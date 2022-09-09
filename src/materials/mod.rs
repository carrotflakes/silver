pub mod checker;
pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod material;
pub mod metal;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use material::Material;
pub use metal::Metal;

use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub enum Basic {
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Lambertian(Lambertian),
    Metal(Metal),
    Checker(checker::Checker<Basic>),
}

impl material::Material for Basic {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3, uv: [f64; 2]) -> Ray {
        match self {
            Basic::Dielectric(dielectric) => dielectric.ray(ray, location, normal, uv),
            Basic::DiffuseLight(diffuse_light) => diffuse_light.ray(ray, location, normal, uv),
            Basic::Lambertian(lambertian) => lambertian.ray(ray, location, normal, uv),
            Basic::Metal(metal) => metal.ray(ray, location, normal, uv),
            Basic::Checker(checker) => checker.ray(ray, location, normal, uv),
        }
    }

    fn color(&self, color: &Vec3, uv: [f64; 2]) -> Vec3 {
        match self {
            Basic::Dielectric(dielectric) => dielectric.color(color, uv),
            Basic::DiffuseLight(diffuse_light) => diffuse_light.color(color, uv),
            Basic::Lambertian(lambertian) => lambertian.color(color, uv),
            Basic::Metal(metal) => metal.color(color, uv),
            Basic::Checker(checker) => checker.color(color, uv),
        }
    }
}
