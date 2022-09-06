use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;

use silver::materials::{Basic as BasicMaterial, *};
use silver::object::Object;
use silver::ray::Ray;
use silver::scene::Scene;
use silver::shapes::{Basic as BasicShape, Sphere};
use silver::vec3::Vec3;
use silver::{camera::Camera, shapes::Triangle};

type MyScene = Scene<BasicShape, BasicMaterial>;

fn linear_to_gamma(v: &Vec3, gamma_factor: f64) -> Vec3 {
    let f = gamma_factor.recip();
    Vec3::new([v.x().powf(f), v.y().powf(f), v.z().powf(f)])
}

#[allow(dead_code)]
fn gamma_to_linear(v: &Vec3, gamma_factor: f64) -> Vec3 {
    Vec3::new([
        v.x().powf(gamma_factor),
        v.y().powf(gamma_factor),
        v.z().powf(gamma_factor),
    ])
}

fn render(
    camera: &Camera,
    scene: &MyScene,
    width: i32,
    height: i32,
    sample: i32,
) -> Vec<Vec<Vec3>> {
    let vec = (0..height)
        .cartesian_product(0..width)
        .collect_vec()
        .into_par_iter()
        .map(|(y, x)| {
            let u: f64 = x as f64 / width as f64;
            let v: f64 = y as f64 / height as f64;
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
            (y, x, linear_to_gamma(&col, 2.2))
        })
        .collect::<Vec<_>>();
    let mut pixels = vec![vec![Vec3::ZERO; width as usize]; height as usize];
    for &(y, x, col) in &vec {
        pixels[y as usize][x as usize] = col;
    }
    pixels
}

fn make_scene_1() -> MyScene {
    Scene {
        objects: vec![
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([0.0, 1000.0, -2.0]), 1000.0)),
                BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.7, 0.7, 0.7]))),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([0.0, -5.0, -7.0]), 5.0)),
                BasicMaterial::Metal(Metal::new(Vec3::new([1.0, 1.0, 1.0]), 0.01)),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([1.3, -0.5, -1.7]), 0.5)),
                BasicMaterial::Metal(Metal::new(Vec3::new([0.2, 1.0, 1.0]), 0.5)),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([0.51, -0.5, -2.0]), 0.5)),
                BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 0.1, 0.1]))),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([-0.51, -0.5, -2.0]), 0.5)),
                BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.1, 0.1, 1.0]))),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([0.0, -0.5, -2.4]), 0.5)),
                BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 1.0, 0.1]))),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([0.0, -0.5, -1.0]), 0.5)),
                BasicMaterial::Dielectric(Dielectric::new(1.1)),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([-1.3, -0.2, -0.0]), 0.2)),
                BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.9, 0.9, 0.9]))),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([-1.3, -0.2, -1.0]), 0.2)),
                BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.9, 0.9, 0.9]))),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([-1.3, -0.2, -2.0]), 0.2)),
                BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.9, 0.9, 0.9]))),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([-1.3, -0.2, -3.0]), 0.2)),
                BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.9, 0.9, 0.9]))),
            ),
            Object::new(
                BasicShape::Sphere(Sphere::new(Vec3::new([-0.8, -0.2, -1.0]), 0.2)),
                BasicMaterial::DiffuseLight(DiffuseLight::new(Vec3::new([3.0, 3.0, 3.0]))),
            ),
        ],
    }
}

fn make_scene_2() -> MyScene {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(13);
    let mut objects = vec![
        Object::new(
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, 1000.0, -2.0]), 1000.0)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.7, 0.7, 0.7]))),
        ),
        Object::new(
            BasicShape::Sphere(Sphere::new(Vec3::new([1.0, -0.5, -1.0]), 0.5)),
            BasicMaterial::Metal(Metal::new(Vec3::new([1.0, 1.0, 1.0]), 0.01)),
        ),
        Object::new(
            BasicShape::Sphere(Sphere::new(Vec3::new([0.4, -0.5, -1.8]), 0.5)),
            BasicMaterial::Dielectric(Dielectric::new(1.6)),
        ),
        Object::new(
            BasicShape::Sphere(Sphere::new(Vec3::new([-0.2, -0.5, -2.6]), 0.5)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 1.0, 1.0]))),
        ),
        Object::new(
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, -1.6, -1.8]), 0.3)),
            BasicMaterial::DiffuseLight(DiffuseLight::new(Vec3::new([3.0, 3.0, 0.0]))),
        ),
        Object::new(
            BasicShape::Triangle(Triangle::new(
                Vec3::new([0.0, 0.0, -0.5]),
                Vec3::new([-0.2, -0.3, -0.5]),
                Vec3::new([0.2, -0.3, -0.5]),
            )),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 0.1, 0.1]))),
        ),
    ];
    for _ in 0..30 {
        objects.push(Object::new(
            BasicShape::Sphere(Sphere::new(
                Vec3::new([rng.gen_range(-3.0..3.0), -0.2, rng.gen_range(-4.0..2.0)]),
                0.2,
            )),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::random(&mut rng))),
        ));
    }
    Scene { objects: objects }
}

fn main() {
    let img_path = "./image.png";

    let width: i32 = 640;
    let height: i32 = 480;
    let camera: Camera = Camera::new(
        &Vec3::new([0.0, -1.0, 2.0]),
        &Vec3::new([0.0, -0.8, 0.0]),
        &Vec3::new([0.0, 1.0, 0.0]),
        60.0f64.to_radians(),
        width as f64 / height as f64,
        0.01,
        3.0,
    );
    let sample: i32 = 20;
    let scene = make_scene_2();

    let start = std::time::Instant::now();
    let pixels = render(&camera, &scene, width, height, sample);
    let end = start.elapsed();
    println!(
        "{}.{:04} elapsed",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );

    let img = image::ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let col = pixels[y as usize][x as usize];
        image::Rgb([
            ((col.r().min(1.0) * 255.99).floor() as u8),
            ((col.g().min(1.0) * 255.99).floor() as u8),
            ((col.b().min(1.0) * 255.99).floor() as u8),
        ])
    });
    img.save(img_path).unwrap();

    println!("done!");
}
