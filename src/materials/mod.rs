pub mod checker;
pub mod constant_medium;
pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;
pub mod tex;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{
    ray::Ray,
    vec3::{NormVec3, Vec3},
};

pub struct RayResult {
    pub emit: Vec3,
    pub albedo: Vec3,
    pub ray: Option<Ray>,
}

pub trait Material {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &NormVec3, uv: [f64; 2]) -> RayResult;
    fn volume(&self) -> Option<(f64, Vec3)> {
        None
    }
}

#[derive(Clone)]
pub enum Basic {
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Lambertian(Lambertian),
    Metal(Metal),
    Checker(checker::Checker<Basic>),
    ConstantMedium(constant_medium::ConstantMedium),
}

impl Material for Basic {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &NormVec3, uv: [f64; 2]) -> RayResult {
        match self {
            Basic::Dielectric(dielectric) => dielectric.ray(ray, location, normal, uv),
            Basic::DiffuseLight(diffuse_light) => diffuse_light.ray(ray, location, normal, uv),
            Basic::Lambertian(lambertian) => lambertian.ray(ray, location, normal, uv),
            Basic::Metal(metal) => metal.ray(ray, location, normal, uv),
            Basic::Checker(checker) => checker.ray(ray, location, normal, uv),
            Basic::ConstantMedium(constant_medium) => {
                constant_medium.ray(ray, location, normal, uv)
            }
        }
    }

    fn volume(&self) -> Option<(f64, Vec3)> {
        match self {
            Basic::Dielectric(dielectric) => dielectric.volume(),
            Basic::DiffuseLight(diffuse_light) => diffuse_light.volume(),
            Basic::Lambertian(lambertian) => lambertian.volume(),
            Basic::Metal(metal) => metal.volume(),
            Basic::Checker(checker) => checker.volume(),
            Basic::ConstantMedium(constant_medium) => constant_medium.volume(),
        }
    }
}
