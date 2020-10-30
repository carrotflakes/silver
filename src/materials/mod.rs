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
}

impl material::Material for Basic {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray {
        match self {
            Basic::Dielectric(dielectric) => dielectric.ray(ray, location, normal),
            Basic::DiffuseLight(diffuse_light) => diffuse_light.ray(ray, location, normal),
            Basic::Lambertian(lambertian) => lambertian.ray(ray, location, normal),
            Basic::Metal(metal) => metal.ray(ray, location, normal),
        }
    }

    fn color(&self, color: &Vec3) -> Vec3 {
        match self {
            Basic::Dielectric(dielectric) => dielectric.color(color),
            Basic::DiffuseLight(diffuse_light) => diffuse_light.color(color),
            Basic::Lambertian(lambertian) => lambertian.color(color),
            Basic::Metal(metal) => metal.color(color),
        }
    }
}
