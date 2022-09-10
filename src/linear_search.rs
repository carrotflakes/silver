use std::ops::Deref;

use crate::bbox::BBox;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::shapes::shape::HitRec;
use crate::shapes::Shape;

struct Object<S: Shape, M: Material, DS: Deref<Target = S>, DM: Deref<Target = M>> {
    shape: DS,
    material: DM,
    bbox: BBox,
}

pub struct LinearSearch<S: Shape, M: Material, DS: Deref<Target = S>, DM: Deref<Target = M>> {
    objects: Vec<Object<S, M, DS, DM>>,
}

impl<S: Shape, M: Material, DS: Deref<Target = S>, DM: Deref<Target = M> + Clone>
    LinearSearch<S, M, DS, DM>
{
    pub fn new(it: impl Iterator<Item = (DS, DM)>) -> Self {
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

    pub fn hit(&self, ray: &Ray) -> Option<(HitRec, DM)> {
        let mut hit: Option<(HitRec, DM)> = None;
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
