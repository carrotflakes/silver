use std::sync::Arc;

use crate::{
    pdf::CosinePdf,
    ray::Ray,
    rng,
    vec3::{NormVec3, Vec3},
};

use super::{Material, RayResult};

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<[u8; 3]>,
}

impl Image {
    pub fn new(width: usize, height: usize, data: Vec<[u8; 3]>) -> Self {
        Image {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, [x, y]: [f32; 2]) -> Vec3 {
        let [r, g, b] =
            self.data[(x as usize % self.width) + (y as usize % self.height) * self.width];
        Vec3::new([r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0])
    }
}

#[derive(Clone)]
pub struct Tex {
    pixel: Arc<dyn Fn([f32; 2]) -> Vec3 + Send + Sync>,
    poses: [[f32; 2]; 3],
}

impl Tex {
    pub fn new(pixel: Arc<dyn Fn([f32; 2]) -> Vec3 + Send + Sync>, poses: [[f32; 2]; 3]) -> Tex {
        Tex {
            pixel,
            poses: [
                [poses[0][0], 1.0 - poses[0][1]],
                [poses[1][0], 1.0 - poses[1][1]],
                [poses[2][0], 1.0 - poses[2][1]],
            ],
        }
    }
}

impl Material for Tex {
    fn ray(&self, _ray: &Ray, location: &Vec3, normal: &NormVec3, uv: [f64; 2]) -> RayResult {
        RayResult {
            emit: Vec3::ZERO,
            albedo: (self.pixel)(uv_to_xy(self.poses, [uv[0] as f32, uv[1] as f32])),
            scattered: Some(Ray::new(
                *location,
                **normal + rng::with(|rng| Vec3::random_in_unit_sphere(rng)),
            )),
            pdf: Some(CosinePdf::new(*normal)),
        }
    }
}

pub fn uv_to_xy(poses: [[f32; 2]; 3], uv: [f32; 2]) -> [f32; 2] {
    let [u, v] = uv;
    [
        poses[0][0] * (1.0 - (u + v)) + poses[1][0] * u + poses[2][0] * v,
        poses[0][1] * (1.0 - (u + v)) + poses[1][1] * u + poses[2][1] * v,
    ]
}
