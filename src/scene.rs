use crate::{materials::material::Material, shapes::Shape};

use super::object::Object;
use super::ray::Ray;
use super::shapes::shape::HitRec;
use super::vec3::Vec3;

pub struct Scene<S: Shape, M: Material> {
    pub objects: Vec<Object<S, M>>,
}

impl<S: Shape, M: Material> Scene<S, M> {
    pub fn ray_(&self, ray: &Ray, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::ZERO;
        }
        let mut hit: Option<(HitRec, &Object<S, M>)> = None;
        let mut time: f64 = std::f64::MAX;
        for object in &self.objects {
            if !object.bbox.should_hit(ray) {
                continue;
            }
            if let Some(hr) = object.shape.hit(ray, 0.001, time) {
                time = hr.time;
                hit = Some((hr, &object));
            }
        }
        match hit {
            Some((
                HitRec {
                    location, normal, ..
                },
                Object { material, .. },
            )) => {
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
