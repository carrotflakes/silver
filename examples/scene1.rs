use silver::camera::Camera;
use silver::materials::checker::Checker;
use silver::materials::{Basic as BasicMaterial, *};
use silver::render::{default_env, render};
use silver::scene::Scene;
use silver::shapes::{Basic as BasicShape, Sphere};
use silver::vec3::Vec3;

fn main() {
    let img_path = "./scene1.png";

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
    let objects = make_scene_1();
    let scene = Scene::new(objects.iter().map(|(s, m)| (s, m)));

    let start = std::time::Instant::now();
    let pixels = render(&camera, |ray| scene.sample(ray, 50, default_env), width, height, sample);
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

fn make_scene_1() -> Vec<(BasicShape, Basic)> {
    vec![
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, 1000.0, -2.0]), 1000.0)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.7, 0.7, 0.7]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, -5.0, -7.0]), 5.0)),
            BasicMaterial::Metal(Metal::new(Vec3::new([1.0, 1.0, 1.0]), 0.01)),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([1.82, -0.5, -1.4]), 0.5)),
            BasicMaterial::Checker(Checker::new(
                Box::new(BasicMaterial::Metal(Metal::new(
                    Vec3::new([0.2, 1.0, 1.0]),
                    0.5,
                ))),
                Box::new(BasicMaterial::Metal(Metal::new(
                    Vec3::new([1.0, 1.0, 0.2]),
                    0.5,
                ))),
            )),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([1.3, -0.5, -1.7]), 0.5)),
            BasicMaterial::Metal(Metal::new(Vec3::new([0.2, 1.0, 1.0]), 0.5)),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.51, -0.5, -2.0]), 0.5)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 0.1, 0.1]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-0.51, -0.5, -2.0]), 0.5)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.1, 0.1, 1.0]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, -0.5, -2.4]), 0.5)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 1.0, 0.1]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, -0.5, -1.0]), 0.5)),
            BasicMaterial::Dielectric(Dielectric::new(1.1)),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-1.3, -0.2, -0.0]), 0.2)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.9, 0.9, 0.9]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-1.3, -0.2, -1.0]), 0.2)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.9, 0.9, 0.9]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-1.3, -0.2, -2.0]), 0.2)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.9, 0.9, 0.9]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-1.3, -0.2, -3.0]), 0.2)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.9, 0.9, 0.9]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-0.8, -0.2, -1.0]), 0.2)),
            BasicMaterial::DiffuseLight(DiffuseLight::new(Vec3::new([3.0, 3.0, 3.0]))),
        ),
    ]
}
