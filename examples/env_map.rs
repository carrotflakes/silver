use silver::{ray::Ray, vec3::Vec3};

pub fn env_map(path: &str) -> impl Fn(&Ray) -> Vec3 {
    let img = image::io::Reader::open(path).unwrap().decode().unwrap();
    let img = img.to_rgb32f();
    let w = img.width() as f64;
    let h = img.height() as f64;

    move |ray| {
        let dir = ray.direction.normalize();
        let x = dir.x();
        let y = dir.y();
        let z = dir.z();
        let a = x.atan2(z) / std::f64::consts::TAU;
        let pixel = img.get_pixel(
            (a * w).rem_euclid(w) as u32,
            ((-y * 0.5 + 0.5) * h).rem_euclid(h) as u32,
        );
        Vec3::new([pixel[0] as f64, pixel[1] as f64, pixel[2] as f64])
    }
}

#[allow(dead_code)]
fn main() {}
