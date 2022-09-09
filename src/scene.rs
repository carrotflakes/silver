use std::ops::Deref;

use crate::bbox::BBox;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::shapes::shape::HitRec;
use crate::shapes::Shape;
use crate::vec3::Vec3;

struct Object<S: Shape, M: Material, DS: Deref<Target = S>, DM: Deref<Target = M>> {
    shape: DS,
    material: DM,
    bbox: BBox,
}

pub struct Scene<
    S: Shape,
    M: Material,
    DS: Deref<Target = S>,
    DM: Deref<Target = M>,
    E: Fn(&Ray) -> Vec3,
> {
    objects: Vec<Object<S, M, DS, DM>>,
    env: E,
}

impl<S: Shape, M: Material, DS: Deref<Target = S>, DM: Deref<Target = M>, E: Fn(&Ray) -> Vec3>
    Scene<S, M, DS, DM, E>
{
    pub fn new(it: impl Iterator<Item = (DS, DM)>, env: E) -> Self {
        Self {
            objects: it
                .map(|(s, m)| Object {
                    bbox: s.bbox(),
                    shape: s,
                    material: m,
                })
                .collect(),
            env,
        }
    }

    fn hit(&self, ray: &Ray) -> Option<(HitRec, &Object<S, M, DS, DM>)> {
        let mut hit: Option<(HitRec, &Object<S, M, DS, DM>)> = None;
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
                location,
                normal,
                uv,
                ..
            },
            Object { material, .. },
        )) = self.hit(ray)
        {
            let r = material.ray(&ray, &location, &normal, uv);
            material.color(&self.ray_(&r, depth - 1), uv)
        } else {
            (self.env)(ray)
        }
    }

    pub fn ray(&self, ray: &Ray) -> Vec3 {
        self.ray_(ray, 50)
    }
}
