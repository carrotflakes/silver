extern crate image;
pub mod vec3;
pub mod ray;
pub mod camera;
pub mod materials;
pub mod shapes;
pub mod object;

use vec3::Vec3;
use ray::Ray;
use camera::Camera;
use shapes::shape::HitRec;
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
                    shape: Box::new(Sphere::new(Vec3(0.0, 100.0, -2.0), 100.0)),
                    material: Box::new(materials::Lambertian::new(Vec3(0.7, 0.7, 0.7)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(0.0, -5.0, -7.0), 5.0)),
                    material: Box::new(materials::Metal::new(0.01))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(1.3, -0.5, -1.7), 0.5)),
                    material: Box::new(materials::Metal::new(0.5))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(0.51, -0.5, -2.0), 0.5)),
                    material: Box::new(materials::Lambertian::new(Vec3(1.0, 0.1, 0.1)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(-0.51, -0.5, -2.0), 0.5)),
                    material: Box::new(materials::Lambertian::new(Vec3(0.1, 0.1, 1.0)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(0.0, -0.5, -2.4), 0.5)),
                    material: Box::new(materials::Lambertian::new(Vec3(1.0, 1.0, 0.1)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(0.1, -0.5, -1.0), 0.5)),
                    material: Box::new(materials::Dielectric::new(1.06))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(-1.3, -0.2, -0.0), 0.2)),
                    material: Box::new(materials::Lambertian::new(Vec3(0.9, 0.9, 0.9)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(-1.3, -0.2, -1.0), 0.2)),
                    material: Box::new(materials::Lambertian::new(Vec3(0.9, 0.9, 0.9)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(-1.3, -0.2, -2.0), 0.2)),
                    material: Box::new(materials::Lambertian::new(Vec3(0.9, 0.9, 0.9)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(-1.3, -0.2, -3.0), 0.2)),
                    material: Box::new(materials::Lambertian::new(Vec3(0.9, 0.9, 0.9)))},
                Object {
                    shape: Box::new(Sphere::new(Vec3(-0.8, -0.2, -1.0), 0.2)),
                    material: Box::new(materials::DiffuseLight::new(Vec3(3.0, 3.0, 3.0)))},
            ]
        }
    }

    pub fn ray_(&self, ray: &Ray, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::ZERO;
        }
        let mut hit: Option<(HitRec, &Object)> = None;
        let mut time: f64 = std::f64::MAX;
        for object in &self.objects {
            if let Some(hr) = object.shape.hit(ray, 0.001, time) {
                time = hr.time;
                hit = Some((hr, &object));
            }
        }
        match hit {
            Some((HitRec {location, normal, ..}, Object {material, ..})) => {
                let r: Ray = material.ray(&ray, &location, &normal);
                material.color(&self.ray_(&r, depth - 1))
            }
            None => {
                let unit_direction: Vec3 = ray.direction.unit_vector();
                let t: f64 = 0.5 * (1.0 - unit_direction.y());
                (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
            }
        }
    }

    pub fn ray(&self, ray: &Ray) -> Vec3 {
        self.ray_(ray, 50)
    }
}

fn linear_to_gamma(v: &Vec3, gamma_factor: f64) -> Vec3 {
    let f = gamma_factor.recip();
    Vec3(v.x().powf(f), v.y().powf(f), v.z() .powf(f))
}

#[allow(dead_code)]
fn gamma_to_linear(v: &Vec3, gamma_factor: f64) -> Vec3 {
    Vec3(v.x().powf(gamma_factor), v.y().powf(gamma_factor), v.z().powf(gamma_factor))
}

fn main() {
    let img_path = "./image.png";

    let width: i64 = 512;
    let height: i64 = 512;
    let camera: Camera = Camera::new(
        &Vec3(0.0, -1.0, 2.0),
        &Vec3(0.0, -0.5, 0.0),
        &Vec3(0.0, 1.0, 0.0),
        60.0f64.to_radians(),
        width as f64 / height as f64,
        0.03,
        4.0
    );
    let sample: i64 = 6;
    let scene = Scene::new();
    let f = |x, y| {
        let u: f64 = x as f64 / width as f64;
        let v: f64 = y as f64 / height as f64;
        let mut col: Vec3 = Vec3::ZERO;
        for dy in 0..sample {
            for dx in 0..sample {
                let du: f64 = ((dx as f64 + 0.5) / sample as f64 - 0.5) / width as f64;
                let dv: f64 = ((dy as f64 + 0.5) / sample as f64 - 0.5) / height as f64;
                let r: Ray = camera.get_ray(u + du, v + dv);
                col = col + scene.ray(&r);
            }
        }
        col = col / sample.pow(2) as f64;
        col = linear_to_gamma(&col, 2.2);
        image::Rgb([
            ((col.r().min(1.0) * 255.99).floor() as u8),
            ((col.g().min(1.0) * 255.99).floor() as u8),
            ((col.b().min(1.0) * 255.99).floor() as u8)])
    };

    let start = std::time::Instant::now();
    let img = image::ImageBuffer::from_fn(width as u32, height as u32, f);
    let end = start.elapsed();
    println!("{}.{:04} elapsed", end.as_secs(), end.subsec_nanos() / 1_000_000);

    img.save(img_path).unwrap();

    println!("done!");
}
