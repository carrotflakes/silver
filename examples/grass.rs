use rand::Rng;
use silver::camera::Camera;
use silver::envs::default_env as env;
use silver::materials::{Lambertian, Material};
use silver::render::render;
use silver::resolvers::bvh::BVH as Resolver;
use silver::shapes::edge::Edge;
use silver::vec3::Vec3;

fn main() {
    let img_path = "./grass.png";

    let width: i32 = 640;
    let height: i32 = 480;
    let camera: Camera = Camera::new(
        &Vec3::new([0.0, 0.8, 1.0]),
        &Vec3::new([0.0, -0.3, 0.0]),
        &Vec3::new([0.0, 1.0, 0.0]),
        39.0f64.to_radians(),
        width as f64 / height as f64,
        0.001,
        1.0,
    );
    let sample_per_pixel: i32 = 10;
    let cutoff = 20;
    let objects = make_scene();
    let scene = Resolver::new(objects.iter().map(|(s, m)| (s, m)));

    let start = std::time::Instant::now();
    let pixels = render(
        &camera,
        |ray| {
            silver::rng::reseed(silver::vec3_to_u64(&ray.direction));
            silver::sample::sample(|r| scene.hit(r), env, ray, cutoff)
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

fn make_scene() -> Vec<(Edge, impl Material)> {
    let v = (0..400)
        .map({
            let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(1);
            move |i| {
                let (x, y) = (i % 20 - 10, i / 20 - 10);
                let (x, y) = (x as f64 * 0.05, y as f64 * 0.05);
                Edge::new(
                    [
                        Vec3::new([x, y, 0.0]),
                        Vec3::new([
                            x + rng.gen_range(-0.01..0.01),
                            y + rng.gen_range(-0.01..0.01),
                            0.5,
                        ]),
                    ],
                    [0.05, 0.0],
                )
            }
        })
        .map(|s| (s, Lambertian::new(Vec3::new([0.05, 0.7, 0.05]))))
        .collect();
    v
}
