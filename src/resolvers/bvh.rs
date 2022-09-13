use crate::bbox::BBox;
use crate::ray::Ray;
use crate::resolvers::Hit;
use crate::shapes::HitRec;
use crate::shapes::Shape;

use super::object::Object;

pub enum BVH<M: Clone, O: Hit<M>> {
    Object {
        object: O,
        _m: std::marker::PhantomData<M>,
    },
    Pair {
        bbox: BBox,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<S: Shape, DS: std::ops::Deref<Target = S> + Clone, M: Clone> BVH<M, Object<DS, M>> {
    pub fn new(it: impl Iterator<Item = (DS, M)>) -> Self {
        let mut objs: Vec<_> = it
            .map(|(s, m)| {
                let bbox = s.bbox();
                Object::new(s, m, bbox)
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
}

impl<M: Clone, O: Hit<M> + AsRef<BBox> + Clone> BVH<M, O> {
    pub fn from_iter(objs: impl Iterator<Item = O>) -> Self {
        let mut objs: Vec<_> = objs.collect();
        Self::build(&mut objs, 0)
    }

    fn build(objs: &mut [O], axis: usize) -> Self {
        match objs.len() {
            0 => panic!(),
            1 => Self::Object {
                object: objs[0].clone(),
                _m: Default::default(),
            },
            n => {
                objs.sort_unstable_by(|a, b| a.as_ref().min[axis].total_cmp(&b.as_ref().min[axis]));
                let left = Self::build(&mut objs[0..n / 2], (axis + 1) % 3);
                let right = Self::build(&mut objs[n / 2..n], (axis + 1) % 3);
                let bbox = left.as_ref().merge(right.as_ref());
                Self::Pair {
                    bbox,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
        }
    }
}

impl<M: Clone, O: Hit<M> + AsRef<BBox>> BVH<M, O> {
    fn hit_(&self, ray: &Ray, tmax: f64, res: &mut Option<(HitRec, M)>) {
        let tmin = 1e-6;
        match self {
            BVH::Object { object, .. } => {
                if !object.as_ref().hit_with_time(ray, tmin, tmax) {
                    return;
                }
                if let Some(hr) = object.hit_with_range(ray, tmin, tmax) {
                    *res = Some(hr);
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

impl<M: Clone, O: Hit<M> + AsRef<BBox>> Hit<M> for BVH<M, O> {
    #[inline]
    fn hit_with_range(&self, ray: &crate::ray::Ray, _tmin: f64, tmax: f64) -> Option<(HitRec, M)> {
        let mut res = None;
        self.hit_(ray, tmax, &mut res);
        res
    }
}

impl<M: Clone, O: Hit<M> + AsRef<BBox>> AsRef<BBox> for BVH<M, O> {
    #[inline]
    fn as_ref(&self) -> &BBox {
        match self {
            BVH::Object { object, .. } => object.as_ref(),
            BVH::Pair { bbox, .. } => &bbox,
        }
    }
}
