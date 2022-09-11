use silver::camera::Camera;
use silver::envs::dark_env as env;
use silver::resolvers::linear_search::LinearSearch as Resolver;
use silver::materials::{Basic as BasicMaterial, *};
use silver::render::render;
use silver::shapes::{Basic as BasicShape, Sphere};
use silver::vec3::Vec3;

fn main() {
    let img_path = "./cornell_box.png";

    let width: i32 = 640;
    let height: i32 = 640;
    let camera: Camera = Camera::new(
        &Vec3::new([0.0, 0.0, 4.0]),
        &Vec3::new([0.0, 0.0, 0.0]),
        &Vec3::new([0.0, 1.0, 0.0]),
        39.0f64.to_radians(),
        width as f64 / height as f64,
        0.001,
        4.0,
    );
    let sample_per_pixel: i32 = 25;
    let cutoff = 20;
    let objects = make_scene();
    let scene = Resolver::new(objects.iter().map(|(s, m)| (s, m)));

    let start = std::time::Instant::now();
    let pixels = render(
        &camera,
        |ray| {
            silver::rng::reseed(silver::vec3_to_u64(&ray.direction));
            silver::sample::sample(|r| scene.hit(r), env, ray, cutoff)
            // silver::sample::sample_with_volume(|r| scene.hit(r), env, ray, cutoff, None)
        },
        width,
        height,
        sample_per_pixel,
    );
    println!("{:?} elapsed", start.elapsed());

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

fn make_scene() -> Vec<(BasicShape, Basic)> {
    let mut v = vec![
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-0.5, 0.3, 0.0]), 0.4)),
            BasicMaterial::Metal(Metal::new(Vec3::new([1.0, 1.0, 1.0]), 0.5)),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.5, 0.5, 0.0]), 0.4)),
            BasicMaterial::Dielectric(Dielectric::new(1.1)),
            // BasicMaterial::ConstantMedium(silver::materials::constant_medium::ConstantMedium::new(4.0, Vec3::new([0.0, 0.0, 1.0]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, -0.8, 0.0]), 0.2)),
            BasicMaterial::DiffuseLight(DiffuseLight::new(Vec3::new([5.0, 5.0, 5.0]))),
        ),
    ];

    v.extend(
        silver::primitives::cube(Vec3::new([0.0, 0.0, 0.0]), Vec3::new([1.0, 1.0, 1.0]))
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i != 3 && *i != 2)
            .map(|(i, t)| {
                (
                    BasicShape::Triangle(t),
                    BasicMaterial::Lambertian(Lambertian::new(match i {
                        4 | 5 => Vec3::new([1.0, 0.0, 0.0]),
                        8 | 9 => Vec3::new([0.0, 1.0, 0.0]),
                        _ => Vec3::new([1.0, 1.0, 1.0]),
                    })),
                )
            }),
    );

    v
}
