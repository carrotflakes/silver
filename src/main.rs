use rand::Rng;

use silver::materials::{Basic as BasicMaterial, *};
use silver::ray::Ray;
use silver::render::{default_env, render};
use silver::scene::Object;
use silver::scene::Scene;
use silver::shapes::{Basic as BasicShape, Sphere};
use silver::vec3::Vec3;
use silver::{camera::Camera, shapes::Triangle};

type MyScene = Scene<BasicShape, BasicMaterial, fn(&Ray) -> Vec3>;

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
        env: default_env,
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
    Scene {
        objects: objects,
        env: default_env,
    }
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
    // let scene = silver::yaml::from_yaml("./scene.yml").unwrap();

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
