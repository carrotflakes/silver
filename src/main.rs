extern crate image;
pub mod vec3;
pub mod ray;
pub mod camera;
pub mod materials;
pub mod shapes;
pub mod object;
pub mod scene;

use itertools::Itertools;
use rayon::prelude::*;
use vec3::Vec3;
use ray::Ray;
use camera::Camera;
use shapes::*;
use materials::*;
use object::Object;
use scene::Scene;

fn linear_to_gamma(v: &Vec3, gamma_factor: f64) -> Vec3 {
    let f = gamma_factor.recip();
    Vec3(v.x().powf(f), v.y().powf(f), v.z() .powf(f))
}

#[allow(dead_code)]
fn gamma_to_linear(v: &Vec3, gamma_factor: f64) -> Vec3 {
    Vec3(v.x().powf(gamma_factor), v.y().powf(gamma_factor), v.z().powf(gamma_factor))
}

fn render(
    camera: &Camera,
    scene: &Scene,
    width: i64,
    height: i64,
    sample: i64) -> Vec<Vec<Vec3>> {
    let vec = (0..height).cartesian_product(0..width).collect_vec().par_iter().map(|(y, x)| {
        let u: f64 = *x as f64 / width as f64;
        let v: f64 = *y as f64 / height as f64;
        let mut col: Vec3 = Vec3::ZERO;
        for dy in 0..sample {
            for dx in 0..sample {
                let du: f64 = ((dx as f64 + 0.5) / sample as f64 - 0.5) / width as f64;
                let dv: f64 = ((dy as f64 + 0.5) / sample as f64 - 0.5) / height as f64;
                let r: Ray = camera.get_ray(u + du, v + dv);
                col = col + scene.ray(&r);
            }
        }
        col = col / sample.pow(2) as f64;
        (*y, *x, linear_to_gamma(&col, 2.2))
    }).collect::<Vec<(i64, i64, Vec3)>>();
    let mut pixels = vec![vec![Vec3::ZERO; width as usize]; height as usize];
    for (y, x, col) in &vec {
        pixels[*y as usize][*x as usize] = *col;
    }
    pixels
}

fn main() {
    let img_path = "./image.png";

    let width: i64 = 512;
    let height: i64 = 512;
    let camera: Camera = Camera::new(
        &Vec3(0.0, -1.0, 2.0),
        &Vec3(0.0, -0.8, 0.0),
        &Vec3(0.0, 1.0, 0.0),
        60.0f64.to_radians(),
        width as f64 / height as f64,
        0.003,
        3.0
    );
    let sample: i64 = 10;
    let scene = Scene {
        objects: vec![
            Object {
                shape: Box::new(Sphere::new(Vec3(0.0, 1000.0, -2.0), 1000.0)),
                material: Box::new(Lambertian::new(Vec3(0.7, 0.7, 0.7)))},
            Object {
                shape: Box::new(Sphere::new(Vec3(0.0, -5.0, -7.0), 5.0)),
                material: Box::new(Metal::new(Vec3(1.0, 1.0, 1.0), 0.01))},
            Object {
                shape: Box::new(Sphere::new(Vec3(1.3, -0.5, -1.7), 0.5)),
                material: Box::new(Metal::new(Vec3(0.2, 1.0, 1.0), 0.5))},
            Object {
                shape: Box::new(Sphere::new(Vec3(0.51, -0.5, -2.0), 0.5)),
                material: Box::new(Lambertian::new(Vec3(1.0, 0.1, 0.1)))},
            Object {
                shape: Box::new(Sphere::new(Vec3(-0.51, -0.5, -2.0), 0.5)),
                material: Box::new(Lambertian::new(Vec3(0.1, 0.1, 1.0)))},
            Object {
                shape: Box::new(Sphere::new(Vec3(0.0, -0.5, -2.4), 0.5)),
                material: Box::new(Lambertian::new(Vec3(1.0, 1.0, 0.1)))},
            Object {
                shape: Box::new(Sphere::new(Vec3(0.0, -0.5, -1.0), 0.5)),
                material: Box::new(Dielectric::new(1.1))},
            Object {
                shape: Box::new(Sphere::new(Vec3(-1.3, -0.2, -0.0), 0.2)),
                material: Box::new(Lambertian::new(Vec3(0.9, 0.9, 0.9)))},
            Object {
                shape: Box::new(Sphere::new(Vec3(-1.3, -0.2, -1.0), 0.2)),
                material: Box::new(Lambertian::new(Vec3(0.9, 0.9, 0.9)))},
            Object {
                shape: Box::new(Sphere::new(Vec3(-1.3, -0.2, -2.0), 0.2)),
                material: Box::new(Lambertian::new(Vec3(0.9, 0.9, 0.9)))},
            Object {
                shape: Box::new(Sphere::new(Vec3(-1.3, -0.2, -3.0), 0.2)),
                material: Box::new(Lambertian::new(Vec3(0.9, 0.9, 0.9)))},
            Object {
                shape: Box::new(Sphere::new(Vec3(-0.8, -0.2, -1.0), 0.2)),
                material: Box::new(DiffuseLight::new(Vec3(3.0, 3.0, 3.0)))},
        ]
    };

    let start = std::time::Instant::now();
    let pixels = render(&camera, &scene, width, height, sample);
    let end = start.elapsed();
    println!("{}.{:04} elapsed", end.as_secs(), end.subsec_nanos() / 1_000_000);

    let img = image::ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let col = pixels[y as usize][x as usize];
        image::Rgb([
            ((col.r().min(1.0) * 255.99).floor() as u8),
            ((col.g().min(1.0) * 255.99).floor() as u8),
            ((col.b().min(1.0) * 255.99).floor() as u8)])
    });
    img.save(img_path).unwrap();

    println!("done!");
}
