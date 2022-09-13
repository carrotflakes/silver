use crate::bbox::BBox;
use crate::resolvers::Hit;
use crate::shapes::HitRec;
use crate::shapes::Shape;

use super::object::Object;

pub struct LinearSearch<M: Clone, O: Hit<M>> {
    objects: Vec<O>,
    _m: std::marker::PhantomData<M>,
}

impl<S: Shape, DS: std::ops::Deref<Target = S> + Clone, M: Clone> LinearSearch<M, Object<DS, M>> {
    pub fn new(it: impl Iterator<Item = (DS, M)>) -> Self {
        Self {
            objects: it
                .map(|(s, m)| {
                    let bbox = s.bbox();
                    Object::new(s, m, bbox)
                })
                .collect(),
            _m: Default::default(),
        }
    }
}

impl<M: Clone, O: Hit<M> + AsRef<BBox>> Hit<M> for LinearSearch<M, O> {
    #[inline]
    fn hit_with_range(
        &self,
        ray: &crate::ray::Ray,
        tmin: f64,
        mut tmax: f64,
    ) -> Option<(HitRec, M)> {
        let mut hit: Option<(HitRec, M)> = None;
        for object in &self.objects {
            if !object.as_ref().should_hit(ray) {
                continue;
            }
            // if !object.bbox.hit_with_time(ray, tmin, tmax) {
            //     continue;
            // }

            if let Some(hr) = object.hit_with_range(ray, tmin, tmax) {
                tmax = hr.0.time;
                hit = Some(hr);
            }
        }
        hit
    }
}
