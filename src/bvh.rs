use std::ops::Deref;

use rand::Rng;

use crate::bbox::BBox;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::shapes::shape::HitRec;
use crate::shapes::Shape;

pub struct Object<S: Clone, M: Clone> {
    shape: S,
    material: M,
    bbox: BBox,
}

impl<S: Clone, M: Clone> Clone for Object<S, M> {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            material: self.material.clone(),
            bbox: self.bbox.clone(),
        }
    }
}

pub enum BVH<S: Clone, M: Clone> {
    Object(Object<S, M>),
    Pair {
        bbox: BBox,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<S: Shape, M: Material, DS: Deref<Target = S> + Clone, DM: Deref<Target = M> + Clone>
    BVH<DS, DM>
{
    pub fn new(it: impl Iterator<Item = (DS, DM)>) -> Self {
        let mut objs: Vec<_> = it
            .map(|(s, m)| Object {
                bbox: s.bbox(),
                shape: s,
                material: m,
            })
            .collect();
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(0);
        let a = Self::build(&mut objs, &mut rng);
        // match &a {
        //     BVH::Object(_) => todo!(),
        //     BVH::Pair { bbox, left, right } => {
        //         dbg!(bbox);
        //         dbg!(left.bbox());
        //         dbg!(right.bbox());
        //     },
        // }
        a
    }

    fn build(objs: &mut [Object<DS, DM>], rng: &mut impl Rng) -> Self {
        match objs.len() {
            0 => panic!(),
            1 => Self::Object(objs[0].clone()),
            n => {
                let axis = rng.gen_range(0..3);
                objs.sort_unstable_by(|a, b| a.bbox.min[axis].total_cmp(&b.bbox.min[axis]));
                let left = Self::build(&mut objs[0..n / 2], rng);
                let right = Self::build(&mut objs[n / 2..n], rng);
                let bbox = left.bbox().merge(&right.bbox());
                Self::Pair {
                    bbox,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
        }
    }

    fn bbox(&self) -> BBox {
        match self {
            BVH::Object(o) => o.bbox.clone(),
            BVH::Pair { bbox, .. } => bbox.clone(),
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<(HitRec, DM)> {
        let mut res = None;
        self.hit_(ray, std::f64::MAX, &mut res);
        res
    }

    fn hit_(&self, ray: &Ray, tmax: f64, res: &mut Option<(HitRec, DM)>) {
        let tmin = 0.001;
        match self {
            BVH::Object(o) => {
                if !o.bbox.hit_with_time(ray, tmin, tmax) {
                    return;
                }
                if let Some(hr) = o.shape.hit(ray, tmin, tmax) {
                    *res = Some((hr, o.material.clone()));
                }
            }
            BVH::Pair { bbox, left, right } => {
                if !bbox.hit_with_time(ray, tmin, tmax) {
                    return;
                }
                left.hit_(ray, tmax, res);
                right.hit_(ray, res.as_ref().map(|r| r.0.time).unwrap_or(tmax), res);
            }
        }
    }
}
