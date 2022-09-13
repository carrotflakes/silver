use crate::bbox::BBox;
use crate::ray::Ray;
use crate::shapes::HitRec;
use crate::shapes::Shape;

use super::Hit;

pub struct Object<S, M: Clone> {
    shape: S,
    material: M,
    bbox: BBox,
}

impl<S, M: Clone> Object<S, M> {
    pub fn new(shape: S, material: M, bbox: BBox) -> Self {
        Self {
            shape,
            material,
            bbox,
        }
    }
}

impl<S: Shape, DS: std::ops::Deref<Target = S> + Clone, M: Clone> Hit<M> for Object<DS, M> {
    #[inline]
    fn hit_with_range(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(HitRec, M)> {
        self.shape
            .hit(ray, tmin, tmax)
            .map(|hr| (hr, self.material.clone()))
    }
}

impl<S, M: Clone> AsRef<BBox> for Object<S, M> {
    #[inline]
    fn as_ref(&self) -> &BBox {
        &self.bbox
    }
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
