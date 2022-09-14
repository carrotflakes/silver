use silver::camera::Camera;
use silver::render::render;
use silver::resolvers::linear_search::LinearSearch;
use silver::vec3::Vec3;

fn main() {
    let img_path = "./yaml.png";

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
    let sample = 20;
    let objects = silver::formats::yaml::load("./scene.yml").unwrap();
    let scene = LinearSearch::new(objects.iter().map(|(s, m)| (s, m)));

    let start = std::time::Instant::now();
    let pixels = render(
        &camera,
        |ray| {
            silver::rng::reseed(silver::util::vec3_to_u64(&ray.direction));
            silver::sample::sample(&scene, silver::envs::default_env, ray, 50)
        },
        width,
        height,
        sample,
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
