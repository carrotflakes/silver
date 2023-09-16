pub mod checker;
pub mod constant_medium;
pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;
pub mod tex;
pub mod uv_map;
pub mod wet_glass;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{
    onb::Onb,
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
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Onb, uv: [f64; 2]) -> RayResult;
    fn volume(&self) -> Option<(f64, Vec3)> {
        None
    }
    fn scattering_pdf(&self, ray: &Ray, normal: &NormVec3, scattered: &Ray) -> f64 {
        let _ = (ray, normal, scattered);
        0.0
    }
}

#[derive(Clone)]
pub enum Basic<'a> {
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Lambertian(Lambertian),
    Metal(Metal),
    Checker(checker::Checker<Basic<'a>>),
    ConstantMedium(constant_medium::ConstantMedium),
    WetGlass(wet_glass::WetGlass),
    UvMap(
        uv_map::UvMap<&'a (dyn Fn(Ray, Vec3, Onb, [f32; 2]) -> (Vec3, Option<Ray>) + Send + Sync)>,
    ),
}

impl<'a> Basic<'a> {
    #[inline]
    pub fn as_ref(&self) -> &dyn Material {
        match self {
            Basic::Dielectric(dielectric) => dielectric,
            Basic::DiffuseLight(diffuse_light) => diffuse_light,
            Basic::Lambertian(lambertian) => lambertian,
            Basic::Metal(metal) => metal,
            Basic::Checker(checker) => checker,
            Basic::ConstantMedium(constant_medium) => constant_medium,
            Basic::WetGlass(wet_glass) => wet_glass,
            Basic::UvMap(uv_map) => uv_map,
        }
    }
}

impl<'a> Material for Basic<'a> {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Onb, uv: [f64; 2]) -> RayResult {
        self.as_ref().ray(ray, location, normal, uv)
    }

    fn volume(&self) -> Option<(f64, Vec3)> {
        self.as_ref().volume()
    }

    fn scattering_pdf(&self, ray: &Ray, normal: &NormVec3, scattered: &Ray) -> f64 {
        self.as_ref().scattering_pdf(ray, normal, scattered)
    }
}
