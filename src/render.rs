use itertools::Itertools;
use rayon::prelude::*;

use crate::{camera::Camera, ray::Ray, vec3::Vec3};

pub fn render(
    camera: &Camera,
    sample: impl (Fn(&Ray) -> Vec3) + Send + Sync,
    width: i32,
    height: i32,
    sample_per_pixel: i32,
) -> Vec<Vec<Vec3>> {
    let vec = (0..height)
        .cartesian_product(0..width)
        .collect_vec()
        .into_par_iter()
        .map(|(y, x)| {
            let u: f64 = x as f64 / width as f64;
            let v: f64 = y as f64 / height as f64;
            let mut col: Vec3 = Vec3::ZERO;
            let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64((x ^ y) as u64);
            for dy in 0..sample_per_pixel {
                for dx in 0..sample_per_pixel {
                    let du: f64 =
                        ((dx as f64 + 0.5) / sample_per_pixel as f64 - 0.5) / width as f64;
                    let dv: f64 =
                        ((dy as f64 + 0.5) / sample_per_pixel as f64 - 0.5) / height as f64;
                    let r: Ray = camera.get_ray(u + du, v + dv, &mut rng);
                    col = col + sample(&r);
                }
            }
            col = col / sample_per_pixel.pow(2) as f64;
            (y, x, linear_to_gamma(&col, 2.2))
        })
        .collect::<Vec<_>>();
    let mut pixels = vec![vec![Vec3::ZERO; width as usize]; height as usize];
    for &(y, x, col) in &vec {
        pixels[y as usize][x as usize] = col;
    }
    pixels
}

pub fn linear_to_gamma(v: &Vec3, gamma_factor: f64) -> Vec3 {
    let f = gamma_factor.recip();
    Vec3::new([v.x().powf(f), v.y().powf(f), v.z().powf(f)])
}

pub fn gamma_to_linear(v: &Vec3, gamma_factor: f64) -> Vec3 {
    Vec3::new([
        v.x().powf(gamma_factor),
        v.y().powf(gamma_factor),
        v.z().powf(gamma_factor),
    ])
}
