use crate::{
    onb::Onb,
    pdf::CosinePdf,
    ray::Ray,
    rng,
    vec3::{NormVec3, Vec3},
};

use super::{Material, RayResult};

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn ray(&self, _ray: &Ray, location: &Vec3, normal: &Onb, _uv: [f64; 2]) -> RayResult {
        // let direction = **normal + rng::with(|rng| *Vec3::random_unit_vector(rng));
        let uvw = Onb::from_w(normal.w());
        let direction = uvw.local(rng::with(|rng| *Vec3::random_cosine_direction(rng)));
        RayResult {
            emit: Vec3::ZERO,
            albedo: self.albedo,
            scattered: Some(Ray::new(*location, direction)),
            pdf: Some(CosinePdf::new(normal.w())),
        }
        // let direction = rng::with(|rng| Vec3::random_in_hemisphere(rng, normal)).normalize();
        // RayResult {
        //     emit: Vec3::ZERO,
        //     albedo: self.albedo,
        //     scattered: Some(Ray::new(*location, *direction)),
        //     pdf: 0.5 / std::f64::consts::PI,
        // }
    }

    fn scattering_pdf(&self, _ray: &Ray, normal: &NormVec3, scattered: &Ray) -> f64 {
        let cosine = normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }
}
