extern crate image;
pub mod vec3;
pub mod ray;
pub mod materials;
pub mod objects;

use vec3::Vec3;
use ray::Ray;
use objects::sphere::Sphere;

//use std::env;

fn make_fn() -> Box<Fn(&Ray) -> Vec3> {
    let spheres = vec![
        Sphere::new(
            Vec3::new(0.0, 0.0, -2.0),
            1.0,
            materials::PlainMat::new(Vec3::new(1.0, 0.0, 0.0))),
        Sphere::new(
            Vec3::new(2.0, 0.0, -2.0),
            0.5,
            materials::PlainMat::new(Vec3::new(1.0, 0.0, 0.0))),
        Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.2,
            materials::PlainMat::new(Vec3::new(1.0, 0.0, 0.0))),
        Sphere::new(
            Vec3::new(0.0, 2.0, -2.0),
            0.5,
            materials::PlainMat::new(Vec3::new(1.0, 0.0, 0.0)))
    ];

    Box::new(move |r: &Ray| -> Vec3 {
        let mut hit: Option<&Sphere> = Option::None;
        let mut hit_dist = std::f64::MAX;
        for shape in &spheres {
            let dist: f64 = shape.hit(r);
            if dist > 0.0 && dist < hit_dist {
                hit = Option::Some(&shape);
                hit_dist = dist;
            }
        }
        match hit {
            Some(shape) => {
                let n = (r.at(hit_dist) - shape.center).unit_vector();
                (n + Vec3::new(1.0, 1.0, 1.0)) * 0.5
                //return sphere.material.color();
            }
            None => {
                let unit_direction: Vec3 = r.direction.unit_vector();
                let t: f64 = 0.5 * (1.0 - unit_direction.y());
                (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
            }
        }
    })
}

fn main() {
    //let mut args = env::args();

    let img_path = "./image.png";

    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let bottom_left: Vec3 = Vec3::new(-2.0, -2.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 4.0, 0.0);
    let rf = make_fn();
    let f = |x, y| {
        let u: f64 = x as f64 / 512 as f64;
        let v: f64 = y as f64 / 512 as f64;
        let r: Ray = Ray::new(origin, bottom_left + u * horizontal + v * vertical);
        let col: Vec3 = rf(&r);
        image::Rgb([
            (col.r() * 255.99).floor() as u8,
            (col.g() * 255.99).floor() as u8,
            (col.b() * 255.99).floor() as u8])
    };
    let img = image::ImageBuffer::from_fn(512, 512, f);

    img.save(img_path).unwrap();

    println!("done!");
}
