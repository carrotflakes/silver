pub mod checker;
pub mod constant_medium;
pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;
pub mod tex;
pub mod wet_glass;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{
    pdf::CosinePdf,
    ray::Ray,
    vec3::{NormVec3, Vec3},
};

pub struct RayResult {
    pub emit: Vec3,
    pub albedo: Vec3,
    pub scattered: Option<Ray>,
    pub pdf: Option<CosinePdf>,
}

pub trait Material {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &NormVec3, uv: [f64; 2]) -> RayResult;
    fn volume(&self) -> Option<(f64, Vec3)> {
        None
    }
    fn scattering_pdf(&self, ray: &Ray, normal: &NormVec3, scattered: &Ray) -> f64 {
        let _ = (ray, normal, scattered);
        0.0
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
    WetGlass(wet_glass::WetGlass),
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
            Basic::WetGlass(wet_glass) => wet_glass.ray(ray, location, normal, uv),
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
            Basic::WetGlass(wet_glass) => wet_glass.volume(),
        }
    }

    fn scattering_pdf(&self, ray: &Ray, normal: &NormVec3, scattered: &Ray) -> f64 {
        match self {
            Basic::Dielectric(dielectric) => dielectric.scattering_pdf(ray, normal, scattered),
            Basic::DiffuseLight(diffuse_light) => {
                diffuse_light.scattering_pdf(ray, normal, scattered)
            }
            Basic::Lambertian(lambertian) => lambertian.scattering_pdf(ray, normal, scattered),
            Basic::Metal(metal) => metal.scattering_pdf(ray, normal, scattered),
            Basic::Checker(checker) => checker.scattering_pdf(ray, normal, scattered),
            Basic::ConstantMedium(constant_medium) => {
                constant_medium.scattering_pdf(ray, normal, scattered)
            }
            Basic::WetGlass(wet_glass) => wet_glass.scattering_pdf(ray, normal, scattered),
        }
    }
}
