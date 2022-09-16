use crate::{
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

    fn get(&self, [x, y]: [f32; 2]) -> Vec3 {
        let [r, g, b] =
            self.data[(x as usize % self.width) + (y as usize % self.height) * self.width];
        Vec3::new([r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0])
    }
}

#[derive(Clone)]
pub struct Tex {
    image: &'static Image,
    poses: [[f32; 2]; 3],
}

impl Tex {
    pub fn new(image: &'static Image, poses: [[f32; 2]; 3]) -> Tex {
        let w = image.width as f32;
        let h = image.height as f32;
        Tex {
            image,
            poses: [
                [(poses[0][0]) * w, (1.0 - poses[0][1]) * h],
                [(poses[1][0]) * w, (1.0 - poses[1][1]) * h],
                [(poses[2][0]) * w, (1.0 - poses[2][1]) * h],
            ],
        }
    }
}

impl Material for Tex {
    fn ray(&self, _ray: &Ray, location: &Vec3, normal: &NormVec3, [u, v]: [f64; 2]) -> RayResult {
        RayResult {
            emit: Vec3::ZERO,
            albedo: crate::util::gamma_to_linear(
                &self.image.get([
                    self.poses[0][0] * (1.0 - (u + v) as f32)
                        + self.poses[1][0] * u as f32
                        + self.poses[2][0] * v as f32,
                    self.poses[0][1] * (1.0 - (u + v) as f32)
                        + self.poses[1][1] * u as f32
                        + self.poses[2][1] * v as f32,
                ]),
                2.2,
            ),
            ray: Some(Ray::new(
                *location,
                **normal + rng::with(|rng| Vec3::random_in_unit_sphere(rng)),
            )),
        }
    }
}
