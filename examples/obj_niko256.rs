use image::GenericImageView;
use silver::camera::Camera;
use silver::render::render;
use silver::resolvers::linear_search::LinearSearch;
use silver::vec3::Vec3;

fn main() {
    let img_path = "./niko256.obj.png";

    let width = 640;
    let height = 480;
    let camera = Camera::new(
        &Vec3::new([1.0, 2.0, 8.0]),
        &Vec3::new([0.0, 0.0, 0.0]),
        &Vec3::new([0.0, 1.0, 0.0]),
        60.0f64.to_radians(),
        width as f64 / height as f64,
        0.001,
        5.0,
    );
    let sample_per_pixel = 100;
    let (faces, _) = silver::formats::obj::load("./niko256.obj");

    let img = image::open("niko256_niko.png").unwrap();

    let image = silver::materials::tex::Image::new(
        img.width() as usize,
        img.height() as usize,
        img.pixels().map(|p| [p.2[0], p.2[1], p.2[2]]).collect(),
    );
    let image = unsafe { std::mem::transmute::<_, &'static silver::materials::tex::Image>(&image) };
    let objects: Vec<_> = faces
        .into_iter()
        .map(|f| {
            (
                // silver::shapes::Triangle::new(transform(f[0].0), transform(f[1].0), transform(f[2].0)),
                silver::shapes::triangle_with_normals::TriangleWithNormals::new(
                    [
                        transform(f.0[0].0),
                        transform(f.0[2].0),
                        transform(f.0[1].0),
                    ],
                    [
                        transform(f.0[0].2),
                        transform(f.0[2].2),
                        transform(f.0[1].2),
                    ],
                ),
                silver::materials::tex::Tex::new(image, [f.0[0].1, f.0[2].1, f.0[1].1]),
            )
        })
        .collect();

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

fn transform(a: [f32; 3]) -> Vec3 {
    Vec3::new([a[0] as f64, a[1] as f64, a[2] as f64])
}
