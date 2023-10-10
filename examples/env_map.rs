use std::f32::consts::{PI, TAU};

use rand::Rng;
use silver::{ray::Ray, vec3::Vec3};

pub fn env_map(
    path: &str,
) -> (
    impl Fn(&Ray) -> Vec3,
    impl Fn(silver::pdf::CosinePdf, Vec3) -> (Vec3, f64),
) {
    let img = image::io::Reader::open(path).unwrap().decode().unwrap();
    let img = img.to_rgb32f();
    let w = img.width() as f64;
    let h = img.height() as f64;

    let env = move |ray: &Ray| {
        let dir = ray.direction.normalize();
        let x = dir.x();
        let y = dir.y();
        let z = dir.z();
        let a = z.atan2(x) / std::f64::consts::TAU;
        let pixel = img.get_pixel(
            (a * w).rem_euclid(w) as u32,
            ((-y * 0.5 + 0.5) * h).rem_euclid(h) as u32,
        );
        Vec3::new([pixel[0] as f64, pixel[1] as f64, pixel[2] as f64])
    };

    let mut areas: Vec<_> = make_voronoi(1000)
        .into_iter()
        .map(|pos| {
            let c = ((pos[0] - 0.5) * PI).cos();
            let direction = Vec3::new([
                ((pos[1] * TAU).cos() * c) as f64,
                (((pos[0] - 0.5) * PI).sin()) as f64,
                ((pos[1] * TAU).sin() * c) as f64,
            ]);
            let e = env(&Ray::new(Vec3::ZERO, direction));
            let weight = e.r() + e.g() + e.b();
            let variance = 0.01;
            (weight, direction, variance)
        })
        .collect();
    let sum = areas.iter().map(|(w, _, _)| w).sum::<f64>();
    areas.iter_mut().for_each(|(w, _, _)| *w /= sum);
    let env_pdf = silver::pdf::EnvPdf::new(areas);
    let pdf_gen = move |p1, _location| {
        let p = silver::pdf::MixturePdf::new(&p1, &env_pdf);
        silver::pdf::Pdf::generate_with_value(&p)
    };

    (env, pdf_gen)
}

fn make_voronoi(num: usize) -> Vec<[f32; 2]> {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(1);
    let mut points: Vec<_> = (0..num)
        .map(|_| [rng.gen::<f32>(), rng.gen::<f32>()])
        .collect();

    let n = 100;
    for _ in 0..n {
        let mut new_points = points.clone();
        for i in 0..points.len() {
            let mut new_point = points[i];
            for j in 0..points.len() {
                if i == j {
                    continue;
                }
                let [x1, y1] = points[i];
                let [x2, y2] = points[j];

                let pos: [f32; 4] = [(y1 - 0.5) * PI, x1 * TAU, (y2 - 0.5) * PI, x2 * TAU];
                let d = distance_on_sphere(pos[0], pos[1], pos[2], pos[3]);
                let [dy1, dx1] = differential_distance_on_sphere(pos[0], pos[1], pos[2], pos[3]);
                let [dx, dy] = [dx1 / TAU, dy1 / PI];
                let r = 0.002 / d.max(1.0).powi(3);

                new_point[0] += dx * r;
                new_point[1] += dy * r;
            }
            new_points[i] = [new_point[0].rem_euclid(1.0), new_point[1].rem_euclid(1.0)];
        }
        points = new_points;
    }
    points
}

fn distance_on_sphere(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    let dlat = (lat2 - lat1).abs();
    let dlon = (lon2 - lon1).abs();
    let a = (dlat * 0.5).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon * 0.5).sin().powi(2);
    2.0 * a.sqrt().asin()
}

fn differential_distance_on_sphere(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> [f32; 2] {
    let diff = 1.0;
    let dlat = (lat2 - lat1).abs();
    let dlon = (lon2 - lon1).abs();
    let dlatsign = (lat2 - lat1).signum();
    let dlonsign = (lon2 - lon1).signum();
    let cc = lat1.cos() * lat2.cos();
    let a = (dlat * 0.5).sin().powi(2) + cc * (dlon * 0.5).sin().powi(2);
    let e = diff / ((1.0 - a.abs()).sqrt() * a.sqrt()).max(1e-8);
    let dlon1 = -(dlonsign * (dlon * 0.5).sin() * (dlon * 0.5).cos() * cc * e);
    let d = (dlat * 0.5).cos() * (dlat * 0.5).sin() * e;
    let dlat1 = -lat1.sin() * lat2.cos() * e * (dlon * 0.5).sin().powi(2) - dlatsign * d;
    [dlat1, dlon1]
}

#[allow(dead_code)]
fn main() {}
