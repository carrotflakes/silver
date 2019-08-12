pub mod material;
pub mod metal;
pub mod lambertian;
pub mod dielectric;
pub mod diffuse_light;

pub use metal::Metal;
pub use lambertian::Lambertian;
pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
