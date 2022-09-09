use rand::Rng;

use silver::materials::checker::Checker;
use silver::materials::{Basic as BasicMaterial, *};
use silver::render::{default_env, render};
use silver::scene::Scene;
use silver::shapes::{Basic as BasicShape, Sphere};
use silver::vec3::Vec3;
use silver::{camera::Camera, shapes::Triangle};

fn main() {
    let img_path = "./scene2.png";

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
    let objects = make_scene_2();
    let scene = Scene::new(objects.iter().map(|(s, m)| (s, m)), default_env);

    let start = std::time::Instant::now();
    let pixels = render(&camera, |ray| scene.ray(ray), width, height, sample);
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

fn make_scene_2() -> Vec<(BasicShape, Basic)> {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(13);
    let mut objects = vec![
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, 1000.0, -2.0]), 1000.0)),
            BasicMaterial::Checker(Checker::new(
                Box::new(BasicMaterial::Lambertian(Lambertian::new(Vec3::new([
                    0.7, 0.7, 0.7,
                ])))),
                Box::new(BasicMaterial::Lambertian(Lambertian::new(Vec3::new([
                    0.8, 0.8, 0.8,
                ])))),
            )),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([1.0, -0.5, -1.0]), 0.5)),
            BasicMaterial::Metal(Metal::new(Vec3::new([1.0, 1.0, 1.0]), 0.01)),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.4, -0.5, -1.8]), 0.5)),
            BasicMaterial::Dielectric(Dielectric::new(1.6)),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-0.2, -0.5, -2.6]), 0.5)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 1.0, 1.0]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, -1.6, -1.8]), 0.3)),
            BasicMaterial::DiffuseLight(DiffuseLight::new(Vec3::new([3.0, 3.0, 0.0]))),
        ),
        (
            BasicShape::Triangle(Triangle::new(
                Vec3::new([0.0, 0.0, -0.5]),
                Vec3::new([-0.2, -0.3, -0.5]),
                Vec3::new([0.2, -0.3, -0.5]),
            )),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 0.1, 0.1]))),
        ),
    ];
    for _ in 0..30 {
        objects.push((
            BasicShape::Sphere(Sphere::new(
                Vec3::new([rng.gen_range(-3.0..3.0), -0.2, rng.gen_range(-4.0..2.0)]),
                0.2,
            )),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::random(&mut rng))),
        ));
    }
    objects
}
