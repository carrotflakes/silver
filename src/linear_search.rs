use std::ops::Deref;

use crate::bbox::BBox;
use crate::ray::Ray;
use crate::shapes::shape::HitRec;
use crate::shapes::Shape;

struct Object<S: Shape, DS: Deref<Target = S>, M> {
    shape: DS,
    material: M,
    bbox: BBox,
}

pub struct LinearSearch<S: Shape, DS: Deref<Target = S>, M: Clone> {
    objects: Vec<Object<S, DS, M>>,
}

impl<S: Shape, DS: Deref<Target = S>, M: Clone> LinearSearch<S, DS, M> {
    pub fn new(it: impl Iterator<Item = (DS, M)>) -> Self {
        Self {
            objects: it
                .map(|(s, m)| Object {
                    bbox: s.bbox(),
                    shape: s,
                    material: m,
                })
                .collect(),
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<(HitRec, M)> {
        let mut hit: Option<(HitRec, M)> = None;
        let mut time: f64 = std::f64::MAX;
        for object in &self.objects {
            if !object.bbox.should_hit(ray) {
                continue;
            }
            // if !object.bbox.hit_with_time(ray, 0.001, time) {
            //     continue;
            // }

            if let Some(hr) = object.shape.hit(ray, 0.001, time) {
                time = hr.time;
                hit = Some((hr, object.material.clone()));
            }
        }
        hit
    }
}
