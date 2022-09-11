use crate::bbox::BBox;
use crate::ray::Ray;
use crate::shapes::HitRec;
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

impl<S: Shape, DS: std::ops::Deref<Target = S> + Clone, M: Clone> BVH<DS, M> {
    pub fn new(it: impl Iterator<Item = (DS, M)>) -> Self {
        let mut objs: Vec<_> = it
            .map(|(s, m)| Object {
                bbox: s.bbox(),
                shape: s,
                material: m,
            })
            .collect();
        let a = Self::build(&mut objs, 0);
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

    fn build(objs: &mut [Object<DS, M>], axis: usize) -> Self {
        match objs.len() {
            0 => panic!(),
            1 => Self::Object(objs[0].clone()),
            n => {
                objs.sort_unstable_by(|a, b| a.bbox.min[axis].total_cmp(&b.bbox.min[axis]));
                let left = Self::build(&mut objs[0..n / 2], (axis + 1) % 3);
                let right = Self::build(&mut objs[n / 2..n], (axis + 1) % 3);
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

    pub fn hit(&self, ray: &Ray) -> Option<(HitRec, M)> {
        let mut res = None;
        self.hit_(ray, std::f64::MAX, &mut res);
        res
    }

    fn hit_(&self, ray: &Ray, tmax: f64, res: &mut Option<(HitRec, M)>) {
        let tmin = 1e-6;
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
