use silver::camera::Camera;
use silver::envs::dark_env as env;
use silver::materials::{Basic as BasicMaterial, *};
use silver::render::render;
use silver::resolvers::linear_search::LinearSearch as Resolver;
use silver::shapes::{Basic as BasicShape, Sphere};
use silver::vec3::Vec3;

fn main() {
    let img_path = "./cornell_box.png";

    let width = 640;
    let height = 640;
    let camera = Camera::new(
        &Vec3::new([0.0, 0.0, 4.0]),
        &Vec3::new([0.0, 0.0, 0.0]),
        &Vec3::new([0.0, 1.0, 0.0]),
        39.0f64.to_radians(),
        width as f64 / height as f64,
        0.001,
        4.0,
    );
    let sample_per_pixel = 25 * 25;
    let cutoff = 10;
    let objects = make_cornell_box();
    let scene = Resolver::new(objects.iter().map(|(s, m)| (s, m)));

    let start = std::time::Instant::now();
    let pixels = render(
        &camera,
        |ray| {
            silver::rng::reseed(silver::util::vec3_to_u64(&ray.direction));
            silver::sample::sample_weighted(&scene, env, ray, cutoff, &objects[2].0)
            // silver::sample::sample_with_volume(
            //     &scene,
            //     env,
            //     ray,
            //     cutoff,
            //     Some((
            //         silver::sample::make_scatter_distance(5.0),
            //         5.0,
            //         Vec3::new([0.9, 0.9, 0.9]),
            //     )),
            // )
        },
        width,
        height,
        sample_per_pixel,
    );
    println!("{:?} elapsed", start.elapsed());

    let img = image::ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let col = silver::util::linear_to_gamma(&pixels[y as usize][x as usize], 2.2);
        image::Rgb([
            ((col.r().min(1.0) * 255.99).floor() as u8),
            ((col.g().min(1.0) * 255.99).floor() as u8),
            ((col.b().min(1.0) * 255.99).floor() as u8),
        ])
    });
    img.save(img_path).unwrap();

    println!("done!");
}

pub fn make_cornell_box() -> Vec<(BasicShape, BasicMaterial)> {
    let mut v = vec![
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-0.5, -0.3, 0.0]), 0.4)),
            BasicMaterial::Metal(Metal::new(Vec3::new([1.0, 1.0, 1.0]), 0.5)),
            // BasicMaterial::Lambertian(Lambertian::new(Vec3::new([0.0, 0.0, 1.0]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.5, -0.5, 0.0]), 0.4)),
            // BasicMaterial::Lambertian(Lambertian::new(Vec3::new([1.0, 1.0, 1.0]))),
            BasicMaterial::Dielectric(Dielectric::new(1.5)),
            // BasicMaterial::ConstantMedium(silver::materials::constant_medium::ConstantMedium::new(4.0, Vec3::new([0.0, 0.0, 1.0]))),
        ),
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([0.0, 0.8, 0.0]), 0.2)),
            BasicMaterial::DiffuseLight(DiffuseLight::new(Vec3::new([5.0, 5.0, 5.0]))),
        ),
        // (
        //     BasicShape::Triangle(silver::shapes::Triangle::new(
        //         Vec3::new([0.0, 0.95, 0.5]),
        //         Vec3::new([-0.25, 0.95, 0.0]),
        //         Vec3::new([0.25, 0.95, 0.0]),
        //     )),
        //     BasicMaterial::DiffuseLight(DiffuseLight::new(Vec3::new([5.0, 5.0, 5.0]))),
        // ),
    ];

    v.extend(
        silver::primitives::cube(Vec3::new([0.0, 0.0, 0.0]), Vec3::new([1.0, 1.0, 1.0]))
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i != 3 && *i != 2)
            .map(|(i, t)| {
                (
                    BasicShape::TriangleBothSide(t.change_both_side()),
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
