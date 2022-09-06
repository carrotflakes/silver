use crate::{materials::material::Material, shapes::Shape};

use super::object::Object;
use super::ray::Ray;
use super::shapes::shape::HitRec;
use super::vec3::Vec3;

pub struct Scene<S: Shape, M: Material, E: Fn(&Ray) -> Vec3> {
    pub objects: Vec<Object<S, M>>,
    pub env: E,
}

impl<S: Shape, M: Material, E: Fn(&Ray) -> Vec3> Scene<S, M, E> {
    fn hit(&self, ray: &Ray) -> Option<(HitRec, &Object<S, M>)> {
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
        hit
    }

    fn ray_(&self, ray: &Ray, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::ZERO;
        }

        if let Some((
            HitRec {
                location, normal, ..
            },
            Object { material, .. },
        )) = self.hit(ray)
        {
            let r = material.ray(&ray, &location, &normal);
            material.color(&self.ray_(&r, depth - 1))
        } else {
            (self.env)(ray)
        }
    }

    pub fn ray(&self, ray: &Ray) -> Vec3 {
        self.ray_(ray, 50)
    }
}
