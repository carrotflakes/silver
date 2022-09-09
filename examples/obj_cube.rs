use silver::camera::Camera;
use silver::materials::Lambertian;
use silver::render::{default_env, render};
use silver::scene::Scene;
use silver::shapes::Triangle;
use silver::vec3::Vec3;

fn main() {
    let img_path = "./cube.obj.png";

    let width: i32 = 640;
    let height: i32 = 480;
    let camera: Camera = Camera::new(
        &Vec3::new([0.0, -3.0, 6.0]),
        &Vec3::new([0.0, 0.0, 0.0]),
        &Vec3::new([0.0, 1.0, 0.0]),
        60.0f64.to_radians(),
        width as f64 / height as f64,
        0.01,
        6.0,
    );
    let sample: i32 = 20;
    let faces = silver::obj::load("./cube.obj");
    let shapes: Vec<_> = faces
        .into_iter()
        .map(|f| Triangle::new(transform(f[0]), transform(f[1]), transform(f[2])))
        .collect();
    let material = Lambertian::new(Vec3::new([0.5, 0.5, 0.5]));
    let scene = Scene::new(shapes.iter().map(|s| (s, &material)), default_env);

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

fn transform(a: [f32; 3]) -> Vec3 {
    Vec3::new([a[0] as f64, -a[1] as f64, a[2] as f64])
}
