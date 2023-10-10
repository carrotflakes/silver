mod env_map;

use rand::Rng;

use silver::materials::{Basic as BasicMaterial, *};
use silver::render::render;
use silver::resolvers::linear_search::LinearSearch;
use silver::shapes::{Basic as BasicShape, Sphere};
use silver::vec3::Vec3;
use silver::{camera::Camera, shapes::Triangle};

fn main() {
    let img_path = "./scene2.png";

    let width = 640;
    let height = 480;
    let camera = Camera::new(
        &Vec3::new([0.0, 1.0, 2.0]),
        &Vec3::new([0.0, 0.8, 0.0]),
        &Vec3::new([0.0, 1.0, 0.0]),
        60.0f64.to_radians(),
        width as f64 / height as f64,
        0.01,
        3.0,
    );
    let sample = 400;
    let objects = make_scene();
    let scene = LinearSearch::new(objects.iter().map(|(s, m)| (s, m)));

    let (env, pdf_gen) = env_map::env_map("qwantani_4k.exr");
    // let env = silver::envs::default_env;

    let start = std::time::Instant::now();
    let pixels = render(
        &camera,
        |ray| {
            silver::rng::reseed(silver::util::vec3_to_u64(ray.direction));
            // silver::sample::sample(&scene, &env, ray, 50)
            silver::sample::sample_weighted(&scene, &env, ray, 50, &pdf_gen)
        },
        width,
        height,
        sample,
    );
    println!("{:?} elapsed", start.elapsed());

    let img = image::ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let col = silver::util::linear_to_gamma(pixels[y as usize][x as usize], 2.2);
        image::Rgb([
            ((col.r().min(1.0) * 255.99).floor() as u8),
            ((col.g().min(1.0) * 255.99).floor() as u8),
            ((col.b().min(1.0) * 255.99).floor() as u8),
        ])
    });
    img.save(img_path).unwrap();

    println!("done!");
}

fn make_scene() -> Vec<(BasicShape, BasicMaterial<'static>)> {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(13);
    let mut objects = vec![
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, -1000.0, -2.0]), 1000.0)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.7, 0.7, 0.7]))),
            // BasicMaterial::Checker(Checker::new(
            //     Box::new(BasicMaterial::Lambertian(Lambertian::new(Vec3::new([
            //         0.7, 0.7, 0.7,
            //     ])))),
            //     Box::new(BasicMaterial::Lambertian(Lambertian::new(Vec3::new([
            //         0.8, 0.8, 0.8,
            //     ])))),
            // )),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([1.0, 0.5, -1.0]), 0.5)),
            BasicMaterial::Metal(Metal::new(Vec3::new([1.0, 1.0, 1.0]), 0.01)),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.4, 0.5, -1.8]), 0.5)),
            BasicMaterial::Dielectric(Dielectric::new(1.6)),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-0.2, 0.5, -2.6]), 0.5)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 1.0, 1.0]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, 1.6, -1.8]), 0.3)),
            BasicMaterial::DiffuseLight(DiffuseLight::new(Vec3::new([3.0, 3.0, 0.0]))),
        ),
        (
            BasicShape::TriangleBothSide(Triangle::new(
                Vec3::new([0.0, 0.0, -0.5]),
                Vec3::new([-0.2, 0.3, -0.5]),
                Vec3::new([0.2, 0.3, -0.5]),
            )),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 0.1, 0.1]))),
        ),
    ];
    for _ in 0..30 {
        objects.push((
            BasicShape::Sphere(Sphere::new(
                Vec3::new([rng.gen_range(-3.0..3.0), 0.2, rng.gen_range(-4.0..2.0)]),
                0.2,
            )),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::random(&mut rng))),
        ));
    }
    objects
}
