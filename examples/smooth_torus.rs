use silver::camera::Camera;
use silver::materials;
use silver::render::render;
use silver::resolvers::linear_search::LinearSearch;
use silver::vec3::Vec3;

fn main() {
    let img_path = "./smooth_torus.obj.png";

    let width = 640;
    let height = 480;
    let camera = Camera::new(
        &Vec3::new([0.5, 1.0, 2.0]),
        &Vec3::new([0.0, -0.2, 0.0]),
        &Vec3::new([0.0, 1.0, 0.0]),
        60.0f64.to_radians(),
        width as f64 / height as f64,
        0.01,
        2.0,
    );
    let sample = 100;
    let (faces, _) = silver::formats::obj::load("./smooth_torus.obj");
    let shapes: Vec<_> = faces
        .into_iter()
        .map(|f| {
            // silver::shapes::Triangle::<true>::new(
            //     transform(f.0[0].0),
            //     transform(f.0[1].0),
            //     transform(f.0[2].0),
            // )
            silver::shapes::triangle_with_normals::TriangleWithNormals::<true>::new(
                [
                    transform(f.0[0].0),
                    transform(f.0[1].0),
                    transform(f.0[2].0),
                ],
                [
                    transform(f.0[0].2),
                    transform(f.0[1].2),
                    transform(f.0[2].2),
                ],
            )
        })
        .collect();
    let material = materials::Lambertian::new(Vec3::new([0.5, 0.5, 0.5]));
    // let material = materials::Metal::new(Vec3::new([0.9, 0.9, 0.9]), 0.98);
    let scene = LinearSearch::new(shapes.iter().map(|s| (s, &material)));

    let start = std::time::Instant::now();
    let pixels = render(
        &camera,
        |ray| {
            silver::rng::reseed(silver::util::vec3_to_u64(&ray.direction));
            silver::sample::sample(&scene, env, ray, 50)
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

fn transform(a: [f32; 3]) -> Vec3 {
    Vec3::new([a[0] as f64, a[1] as f64, a[2] as f64])
}

pub fn env(ray: &silver::ray::Ray) -> Vec3 {
    let direction = ray.direction.normalize();
    let t = 0.5 * (1.0 - direction.y());
    (1.0 - t) * Vec3::new([0.25, 0.7, 1.0]) + t * Vec3::new([1.0, 0.1, 0.1])
}
