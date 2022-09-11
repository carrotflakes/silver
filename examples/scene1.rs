use silver::camera::Camera;
use silver::linear_search::LinearSearch;
use silver::materials::checker::Checker;
use silver::materials::{Basic as BasicMaterial, *};
use silver::render::render;
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
        0.001,
        3.0,
    );
    let sample: i32 = 20;
    let objects = make_scene_1();
    let scene = LinearSearch::new(objects.iter().map(|(s, m)| (s, m)));

    let start = std::time::Instant::now();
    let pixels = render(
        &camera,
        |ray| {
            silver::rng::reseed(silver::vec3_to_u64(&ray.direction));
            // silver::sample::sample(|r| scene.hit(r), silver::envs::default_env, ray, 20)

            silver::sample::sample_with_volume(
                |r| scene.hit(r),
                silver::envs::fancy_env,
                ray,
                20,
                None,
            )
        },
        width,
        height,
        sample,
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

fn make_scene_1() -> Vec<(BasicShape, Basic)> {
    let mut v = vec![
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
            BasicShape::Sphere(Sphere::new(Vec3::new([1.3, -1.3, -1.0]), 0.6)),
            BasicMaterial::ConstantMedium(constant_medium::ConstantMedium::new(
                2.0,
                Vec3::new([0.95, 0.95, 0.95]),
            )),
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
    ];

    v.extend(
        silver::primitives::tetrahedron(Vec3::new([-2.0, -1.3, -2.]), 0.4)
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                (
                    BasicShape::Triangle(t),
                    BasicMaterial::Lambertian(Lambertian::new(Vec3::new([
                        i as f64 * 0.3,
                        1.0 - i as f64 * 0.3,
                        0.0,
                    ]))),
                )
            }),
    );

    v.extend(
        silver::primitives::cube(Vec3::new([1.0, -0.3, 0.0]), Vec3::new([0.2, 0.2, 0.2]))
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                (
                    BasicShape::Triangle(t),
                    BasicMaterial::Lambertian(Lambertian::new(Vec3::new([
                        i as f64 * 0.05,
                        1.0 - i as f64 * 0.05,
                        (i % 2) as f64 * 0.5,
                    ]))),
                )
            }),
    );

    v
}
