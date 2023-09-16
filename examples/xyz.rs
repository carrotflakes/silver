use silver::camera::Camera;
use silver::materials::{Basic as BasicMaterial, *};
use silver::render::render;
use silver::resolvers::linear_search::LinearSearch;
use silver::shapes::{edge, Basic as BasicShape, Sphere};
use silver::vec3::Vec3;

fn main() {
    let img_path = "./xyz.png";

    let width = 640;
    let height = 480;
    let camera = Camera::new(
        &Vec3::new([0.1, 0.1, 1.5]),
        &Vec3::new([0.0, 0.0, 0.0]),
        &Vec3::new([0.0, 1.0, 0.0]),
        60.0f64.to_radians(),
        width as f64 / height as f64,
        0.001,
        1.5,
    );
    let sample_per_pixel = 20;
    let cutoff = 20;
    let objects = make_scene();
    let scene = LinearSearch::new(objects.iter().map(|(s, m)| (s, m)));

    let start = std::time::Instant::now();
    let pixels = render(
        &camera,
        |ray| {
            silver::rng::reseed(silver::util::vec3_to_u64(ray.direction));
            silver::sample::sample(&scene, silver::envs::default_env, ray, cutoff)
        },
        width,
        height,
        sample_per_pixel,
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

fn make_scene() -> Vec<(BasicShape, Basic<'static>)> {
    vec![
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, 0.0, 0.0]), 0.2)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.7, 0.7, 0.7]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.5, 0.0, 0.0]), 0.1)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 0.0, 0.0]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, 0.5, 0.0]), 0.1)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.0, 1.0, 0.0]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, 0.0, 0.5]), 0.1)),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.0, 0.0, 1.0]))),
        ),
        (
            BasicShape::Edge(edge::Edge::new(
                [Vec3::new([0.0, 0.0, 0.0]), Vec3::new([0.5, 0.0, 0.0])],
                [0.02, 0.02],
            )),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 0.0, 0.0]))),
        ),
        (
            BasicShape::Edge(edge::Edge::new(
                [Vec3::new([0.0, 0.0, 0.0]), Vec3::new([0.0, 0.5, 0.0])],
                [0.02, 0.02],
            )),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.0, 1.0, 0.0]))),
        ),
        (
            BasicShape::Edge(edge::Edge::new(
                [Vec3::new([0.0, 0.0, 0.0]), Vec3::new([0.0, 0.0, 0.5])],
                [0.02, 0.02],
            )),
            BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.0, 0.0, 1.0]))),
        ),
    ]
}
