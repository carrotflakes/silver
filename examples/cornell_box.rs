mod env_map;

use silver::camera::Camera;
#[allow(unused_imports)]
use silver::envs::dark_env as env;
use silver::materials::uv_map::UvMap;
use silver::materials::{Basic as BasicMaterial, *};
use silver::ray::Ray;
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
        // &Vec3::new([2.0, 1.0, 4.0]),
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

    // let env = env_map::env_map("qwantani_4k.exr");

    let pdf_gen = |p1, location| {
        let p2 = silver::pdf::ShapePdf::new(location, &objects[2].0);
        let p = silver::pdf::MixturePdf::new(&p1, &p2);
        silver::pdf::Pdf::generate_with_value(&p)
    };

    let start = std::time::Instant::now();
    let pixels = render(
        &camera,
        |ray| {
            silver::rng::reseed(silver::util::vec3_to_u64(ray.direction));
            silver::sample::sample_weighted(&scene, &env, ray, cutoff, &pdf_gen)
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

pub fn make_cornell_box() -> Vec<(BasicShape, BasicMaterial<'static>)> {
    let mut v = vec![
        (
            BasicShape::Sphere(Sphere::new(Vec3::new([-0.5, -0.3, 0.0]), 0.4)),
            BasicMaterial::Metal(Metal::new(Vec3::new([1.0, 1.0, 1.0]), 0.0)),
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

    if false {
        let z = 2.0;
        for i in 0..20 {
            let x = (i * 2 % 31) as f64 / 31.0 - 0.5;
            let y = ((i * (i + 23) + 3) % 31) as f64 / 31.0 - 0.5;
            let s = ((i % 3) as f64 / 3.0 + 1.0) * 0.03;
            v.push(wet_glass([
                Vec3::new([x, y, z]),
                Vec3::new([x - s, y + s, z]),
                Vec3::new([x + s, y + s, z]),
            ]))
        }
    }

    // fn uv_map_fn(
    //     ray: Ray,
    //     location: Vec3,
    //     normal: silver::onb::Onb,
    //     uv: [f32; 2],
    // ) -> (Vec3, Option<Ray>) {
    //     // let x = (uv[0] as f64 - 0.5) * 2.0;
    //     // let y = -(uv[1] as f64 - 0.5) * 2.0;
    //     let x = (uv[0] as f64 * 10.0 % 2.0 - 1.0) * 2.0;
    //     let y = -(uv[1] as f64 * 10.0 % 2.0 - 1.0) * 2.0;

    //     let dir = ray.direction;
    //     // let dir = dir + silver::rng::with(|rng| Vec3::random_in_unit_sphere(rng) * 0.1);
    //     let r = (x).hypot(y);
    //     let r = if r < 1.0 { r.powf(2.0) * 0.5 } else { 0.0 };
    //     let dir = *dir.normalize() * (1.0 - r) + *Vec3::new([x, y, 0.001]).normalize() * r;
    //     let color = Vec3::new([1.0; 3]);
    //     (color, Some(Ray::new(location, dir)))
    // }
    // v.push((
    //     BasicShape::TriangleBothSide(silver::shapes::Triangle::new(
    //         Vec3::new([-1.0, -1.0, 1.0]),
    //         Vec3::new([1.0, -1.0, 1.0]),
    //         Vec3::new([-1.0, 1.0, 1.0]),
    //     )),
    //     BasicMaterial::UvMap(UvMap::new(&uv_map_fn, [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]])),
    // ));
    // v.push((
    //     BasicShape::TriangleBothSide(silver::shapes::Triangle::new(
    //         Vec3::new([-1.0, 1.0, 1.0]),
    //         Vec3::new([1.0, -1.0, 1.0]),
    //         Vec3::new([1.0, 1.0, 1.0]),
    //     )),
    //     BasicMaterial::UvMap(UvMap::new(&uv_map_fn, [[0.0, 1.0], [1.0, 0.0], [1.0, 1.0]])),
    // ));

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

fn wet_glass(vs: [Vec3; 3]) -> (BasicShape, BasicMaterial<'static>) {
    (
        BasicShape::TriangleBothSide(silver::shapes::Triangle::new(vs[0], vs[1], vs[2])),
        BasicMaterial::WetGlass(wet_glass::WetGlass::new(
            vs[0] * 0.5 + vs[1] * 0.25 + vs[2] * 0.25,
        )),
    )
}
