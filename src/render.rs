use rand::Rng;
use rayon::prelude::*;

use crate::{camera::Camera, ray::Ray, rng::MainRng, vec3::Vec3};

pub fn render(
    camera: &Camera,
    sample: impl (Fn(&Ray) -> Vec3) + Send + Sync,
    width: i32,
    height: i32,
    sample_per_pixel: i32,
) -> Vec<Vec<Vec3>> {
    let vec = (0..height * width)
        .map(|i| (i / width, i % width))
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(y, x)| {
            let u = x as f64 / width as f64;
            let v = y as f64 / height as f64;
            let mut color = Vec3::ZERO;
            let mut rng: MainRng = rand::SeedableRng::seed_from_u64((x ^ y) as u64);
            for _ in 0..sample_per_pixel {
                let dx = rng.gen::<f64>();
                let dy = rng.gen::<f64>();
                let du = (dx - 0.5) / width as f64;
                let dv = (dy - 0.5) / height as f64;
                let r = camera.get_ray(u + du, 1.0 - (v + dv), &mut rng);
                color = color + sample(&r);
            }
            color = color / sample_per_pixel as f64;
            (y, x, color)
        })
        .collect::<Vec<_>>();
    let mut pixels = vec![vec![Vec3::ZERO; width as usize]; height as usize];
    for &(y, x, color) in &vec {
        pixels[y as usize][x as usize] = color;
    }
    pixels
}
