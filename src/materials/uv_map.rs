use crate::{onb::Onb, ray::Ray, vec3::Vec3};

use super::{tex::uv_to_xy, Material, RayResult};

#[derive(Clone)]
pub struct UvMap<F: Fn(Ray, Vec3, Onb, [f32; 2]) -> (Vec3, Option<Ray>) + Send + Sync> {
    pixel: F,
    poses: [[f32; 2]; 3],
}

impl<F: Fn(Ray, Vec3, Onb, [f32; 2]) -> (Vec3, Option<Ray>) + Send + Sync> UvMap<F> {
    pub fn new(pixel: F, poses: [[f32; 2]; 3]) -> Self {
        UvMap {
            pixel,
            poses: [
                [poses[0][0], 1.0 - poses[0][1]],
                [poses[1][0], 1.0 - poses[1][1]],
                [poses[2][0], 1.0 - poses[2][1]],
            ],
        }
    }
}

impl<F: Fn(Ray, Vec3, Onb, [f32; 2]) -> (Vec3, Option<Ray>) + Send + Sync> Material for UvMap<F> {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Onb, uv: [f64; 2]) -> RayResult {
        let (albedo, scattered) = (self.pixel)(
            *ray,
            *location,
            *normal,
            uv_to_xy(self.poses, [uv[0] as f32, uv[1] as f32]),
        );

        RayResult {
            emit: Vec3::ZERO,
            albedo,
            scattered,
            // pdf: Some(CosinePdf::new(*normal)),
            pdf: None,
        }
    }
}
