use rand::Rng;
use silver::camera::Camera;
use silver::envs::default_env as env;
use silver::materials::{Basic as BasicMaterial, *};
use silver::render::render;
use silver::resolvers::bvh::BVH as Resolver;
use silver::shapes::{Basic as BasicShape, Sphere};
use silver::vec3::Vec3;

fn main() {
    let img_path = "./bubbles.png";

    let width = 640;
    let height = 480;
    let camera = Camera::new(
        &Vec3::new([0.0, 0.0, 1.0]),
        &Vec3::new([0.0, 0.0, 0.0]),
        &Vec3::new([0.0, 1.0, 0.0]),
        39.0f64.to_radians(),
        width as f64 / height as f64,
        0.001,
        1.0,
    );
    let sample_per_pixel = 100;
    let cutoff = 20;
    let objects = make_scene();
    let scene = Resolver::new(objects.iter().map(|(s, m)| (s, m)));

    let start = std::time::Instant::now();
    let pixels = render(
        &camera,
        |ray| {
            silver::rng::reseed(silver::util::vec3_to_u64(ray.direction));
            silver::sample::sample(&scene, env, ray, cutoff)
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

fn make_scene() -> Vec<(BasicShape, BasicMaterial)> {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(1);
    let radius = 0.08;
    let mut bs: Vec<_> = (0..500)
        .map(|_| {
            Vec3::new([
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ])
        })
        .collect();

    for _ in 0..10 {
        for i in 0..bs.len() {
            for j in 0..bs.len() {
                if i != j && (bs[i] - bs[j]).norm() < radius * 2.1 {
                    bs[i] = bs[i] - (bs[j] - bs[i]) * 0.5;
                }
            }
        }
    }

    let v = bs
        .iter()
        .map(|p| {
            (
                BasicShape::Sphere(Sphere::new(p.clone(), radius)),
                match rng.gen_range(0..100) {
                    i if i < 50 => {
                        BasicMaterial::Lambertian(Lambertian::new(Vec3::random(&mut rng)))
                    }
                    i if i < 80 => BasicMaterial::Metal(Metal::new(
                        Vec3::random(&mut rng),
                        rng.gen_range(0.0..0.9f64).powi(2),
                    )),
                    i if i < 99 => {
                        BasicMaterial::Dielectric(Dielectric::new(rng.gen_range(0.9..1.2)))
                    }
                    _ => BasicMaterial::DiffuseLight(DiffuseLight::new(Vec3::new([1.0, 1.0, 1.0]))),
                },
            )
        })
        .collect();
    v
}
