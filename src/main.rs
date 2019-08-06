extern crate image;
pub mod vec3;
pub mod ray;
pub mod materials;
pub mod shapes;
pub mod object;

use vec3::Vec3;
use ray::Ray;
use shapes::shape::{HitRec};
use shapes::sphere::Sphere;
use object::Object;

struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: vec![
                Object {
                    shape: Box::new(Sphere::new(Vec3::new(0.0, 0.0, -2.0), 1.0)),
                    material: Box::new(materials::PlainMat::new(Vec3::new(1.0, 0.0, 0.0)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3::new(2.0, 0.0, -2.0), 0.5)),
                    material: Box::new(materials::PlainMat::new(Vec3::new(1.0, 0.0, 0.0)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.2)),
                    material: Box::new(materials::PlainMat::new(Vec3::new(1.0, 0.0, 0.0)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3::new(1.5, 1.5, -2.0), 0.5)),
                    material: Box::new(materials::PlainMat::new(Vec3::new(1.0, 0.0, 0.0)))}
            ]
        }
    }

    pub fn ray_(&self, ray: &Ray, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::ZERO;
        }
        let mut hit: Option<(HitRec, &Object)> = Option::None;
        let mut time: f64 = std::f64::MAX;
        for object in &self.objects {
            match object.shape.hit(ray) {
                Option::Some(hr) =>
                    if hr.time > 0.0 && hr.time < time {
                        time = hr.time;
                        hit = Option::Some((hr, &object));
                    },
                Option::None => ()
            }
        }
        match hit {
            Some((HitRec {location, normal, ..}, Object {..})) => {
                let b: Vec3 = -(ray.direction.dot(&normal)) * normal;
                let r: Ray = Ray::new(location, ray.direction + 2.0 * b);
                self.ray_(&r, depth - 1) * 0.8
                //(n + Vec3::new(1.0, 1.0, 1.0)) * 0.5
                //return sphere.material.color();
            }
            None => {
                let unit_direction: Vec3 = ray.direction.unit_vector();
                let t: f64 = 0.5 * (1.0 - unit_direction.y());
                (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
            }
        }
    }

    pub fn ray(&self, ray: &Ray) -> Vec3 {
        self.ray_(ray, 10)
    }
}

fn linear_to_gamma(v: &Vec3, gamma_factor: f64) -> Vec3 {
    let f = gamma_factor.recip();
    Vec3::new(v.x().powf(f), v.y().powf(f), v.z() .powf(f))
}

#[allow(dead_code)]
fn gamma_to_linear(v: &Vec3, gamma_factor: f64) -> Vec3 {
    Vec3::new(v.x().powf(gamma_factor), v.y().powf(gamma_factor), v.z().powf(gamma_factor))
}

fn main() {
    let img_path = "./image.png";

    let width: i64 = 512;
    let height: i64 = 512;
    let origin: Vec3 = Vec3::new(0.0, 0.0, 2.0);
    let bottom_left: Vec3 = Vec3::new(-1.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(2.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let sample: i64 = 2;
    let scene = Scene::new();
    let f = |x, y| {
        let u: f64 = x as f64 / width as f64;
        let v: f64 = y as f64 / height as f64;
        let d: Vec3 = bottom_left + u * horizontal + v * vertical;
        let mut col: Vec3 = Vec3::ZERO;
        for dy in 0..sample {
            for dx in 0..sample {
                let dd: Vec3 = ((dx as f64 + 0.5) / sample as f64 - 0.5) / width as f64 * horizontal + ((dy as f64 + 0.5) / sample as f64 - 0.5) / height as f64 * vertical;
                let r: Ray = Ray::new(origin, d + dd);
                col = col + scene.ray(&r);
            }
        }
        col = col / sample.pow(2) as f64;
        col = linear_to_gamma(&col, 2.2);
        image::Rgb([
            (col.r() * 255.99).floor() as u8,
            (col.g() * 255.99).floor() as u8,
            (col.b() * 255.99).floor() as u8])
    };
    let img = image::ImageBuffer::from_fn(width as u32, height as u32, f);

    img.save(img_path).unwrap();

    println!("done!");
}
